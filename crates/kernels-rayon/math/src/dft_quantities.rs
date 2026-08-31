//! DFT-specific derived quantities.
//!
//! Wigner-Seitz radius, reduced gradient, Thomas-Fermi kinetic energy density,
//! and dimensionless inhomogeneity parameter alpha.

// `rmath` below is `crate::rmath` -- this crate's BitExact surface, not the
// upstream crate, whose free functions are deliberately the Fast path.
use crate::rmath;

use super::constants::{RS_CONST, KF_CONST};
use super::powers::{pow_1_3, pow_4_3, pow_5_3};

/// Wigner-Seitz radius: rs = RS_CONST * rho^(-1/3) = RS_CONST / cbrt(rho).
///
/// RS_CONST = (3/(4*pi))^(1/3)
pub fn wigner_seitz_rs(rho: f64) -> f64 {
    (RS_CONST as f64) * pow_1_3(1.0_f64 / rho)
}

/// Reduced density gradient: s = sqrt(sigma) / (2 * kf * rho^(4/3))
///
/// where kf = KF_CONST * rho^(1/3) so 2*kf*rho^(4/3) = 2*KF_CONST*rho^(1/3)*rho^(4/3) = 2*KF_CONST*rho^(5/3)
/// Actually, the standard form is: s = |grad rho| / (2 * kF * rho) where kF = (3*pi^2*rho)^(1/3)
/// So s = sqrt(sigma) / (2 * KF_CONST * rho^(4/3))
pub fn reduced_gradient_s(rho: f64, sigma: f64) -> f64 {
    rmath::sqrt(sigma) / (2.0_f64 * (KF_CONST as f64) * pow_4_3(rho))
}

/// Thomas-Fermi kinetic energy density: t_TF = (3/10) * (3*pi^2)^(2/3) * rho^(5/3)
///
/// Note: (3*pi^2)^(2/3) = KF_CONST^2
pub fn tf_kinetic(rho: f64) -> f64 {
    0.3_f64 * (KF_CONST as f64) * (KF_CONST as f64) * pow_5_3(rho)
}

/// Dimensionless inhomogeneity parameter alpha:
/// alpha = (tau - tau_W) / tau_TF
///
/// where tau_W = sigma/(8*rho) is the von Weizsacker kinetic energy density
/// and tau_TF is the Thomas-Fermi kinetic energy density.
///
/// alpha = 1 for the uniform electron gas (tau = tau_TF, sigma = 0).
pub fn dimensionless_alpha(rho: f64, sigma: f64, tau: f64) -> f64 {
    let tau_w = sigma / (8.0_f64 * rho);
    let tau_tf = tf_kinetic(rho);
    (tau - tau_w) / tau_tf
}
