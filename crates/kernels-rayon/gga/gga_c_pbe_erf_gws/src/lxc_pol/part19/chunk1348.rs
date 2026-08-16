//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1348/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1348(t3854: f64, t898: f64, t13796: f64, t13798: f64, t3989: f64, t57321: f64, t875: f64, t14724: f64, t3306: f64, t343: f64, t12206: f64, t3965: f64) -> (f64, f64, f64, f64) {
    let t57728 = t898 * t3854;
    let t57731 = t3989 * t13796 * t57728 * t13798;
    let t57740 = t3989 * t13796 * t57321 * t875;
    let t57745 = t3989 * t13796 * t14724 * t343 * t3306;
    let t57747 = t3965 * t12206;
    (t57731, t57740, t57745, t57747)
}
