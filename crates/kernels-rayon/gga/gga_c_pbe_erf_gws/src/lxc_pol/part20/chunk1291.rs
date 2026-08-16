//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1291/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1291(t13796: f64, t3722: f64, t3989: f64, t875: f64, t14637: f64, t52926: f64, t9872: f64, t13917: f64, t3258: f64, t3757: f64, t51021: f64, t938: f64) -> (f64, f64, f64) {
    let t56431 = t3989 * t13796 * t3722 * t875;
    let t56434 = t14637 * t52926 * t9872;
    let t56439 = t13917 * t51021 * t3258 * t3757 * t938;
    (t56431, t56434, t56439)
}
