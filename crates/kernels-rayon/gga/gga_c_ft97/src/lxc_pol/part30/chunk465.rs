//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 465/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk465(t7440: f64, t7515: f64, t7511: f64, t7512: f64, t2506: f64, t1434: f64, t193: f64, t743: f64, t7484: f64, t2372: f64, t27: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7516 = t7515 * t7440;
    let t7518 = t7511 * t7512 * t7516;
    let t7520 = t2506 * t7440;
    let t7522 = t1434 * t193 * t7520;
    let t7524 = t743 * t7484;
    let t7526 = t1434 * t193 * t7524;
    let t7528 = t2372 * t7440;
    let t7530 = t89 * t27 * t7528;
    (t7516, t7518, t7520, t7522, t7524, t7526, t7528, t7530)
}
