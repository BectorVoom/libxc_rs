//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1340/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1340(t2108: f64, t2240: f64, t2244: f64, t39049: f64, t7245: f64, t24525: f64, t9231: f64, t24503: f64, t33: f64, t2110: f64, t22519: f64, t22527: f64, t24505: f64, t24508: f64, t24520: f64, t24526: f64, t6492: f64, t6495: f64, t7256: f64, t7259: f64, t83748: f64) -> f64 {
    let t85507 = t2240 * t2244 * t2108;
    let t85510 = t39049 * t7245;
    let t85514 = t9231 * t24525;
    let t85524 = t2240 * t33 * t24503;
    let t85532 = -5.0_f64 * t85507 * t6492 + 5.0_f64 / 2.0_f64 * t85510 * t6492 + t83748 * t2110 + 5.0_f64 * t85514 * t6492 + 2.0_f64 * t22519 * t7256 + 5.0_f64 * t24520 * t22527 + 2.0_f64 * t22519 * t7259 + 5.0_f64 / 2.0_f64 * t85524 * t6492 + t6495 * t24505 + 5.0_f64 * t24526 * t22527 + 2.0_f64 * t6495 * t24508;
    t85532
}
