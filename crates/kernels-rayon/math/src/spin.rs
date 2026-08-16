//! Spin polarization transforms for DFT calculations.
//!
//! Provides functions for computing total density, spin polarization (zeta),
//! spin-scaling factor, and clamping zeta to valid range.
//! Generic over `` to support both f64 and f32.

use super::powers::pow_4_3;

/// Compute total density from spin-up and spin-down densities.
///
/// Returns `rho_up + rho_down`.
pub fn compute_total(rho_up: f64, rho_down: f64) -> f64 {
    rho_up + rho_down
}

/// Compute spin polarization zeta = (rho_up - rho_down) / (rho_up + rho_down).
///
/// If total density is below `threshold`, returns 0.0 (unpolarized).
pub fn compute_zeta(rho_up: f64, rho_down: f64, threshold: f64) -> f64 {
    let total = rho_up + rho_down;
    let zeta = (rho_up - rho_down) / total;
    (if total < threshold { 0.0_f64 } else { zeta })
}

/// Combined total+zeta computation (convenience wrapper).
/// Returns total density. Use `compute_zeta` separately for zeta.
pub fn to_total_zeta_total(rho_up: f64, rho_down: f64) -> f64 {
    compute_total(rho_up, rho_down)
}

/// Spin scaling factor: f(zeta) = ((1+zeta)^(4/3) + (1-zeta)^(4/3)) / 2.
///
/// Approaches 1.0 for unpolarized (zeta=0) and 2^(1/3) for fully polarized (zeta=1).
pub fn spin_scaling(zeta: f64) -> f64 {
    let up = 1.0_f64 + zeta;
    let down = 1.0_f64 - zeta;
    (pow_4_3(up) + pow_4_3(down)) / 2.0_f64
}

/// Clamp zeta to [-(1-threshold), (1-threshold)].
///
/// Prevents division by zero in spin-dependent quantities when
/// one spin channel has nearly zero density.
pub fn clamp_zeta(zeta: f64, threshold: f64) -> f64 {
    let upper = 1.0_f64 - threshold;
    let lower = -(1.0_f64 - threshold);
    let clamped = (if zeta > upper { upper } else { zeta });
    (if clamped < lower { lower } else { clamped })
}
