//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1312/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1312(t3959: f64, t9932: f64, t3897: f64, t4386: f64, t13792: f64, t15167: f64, t3972: f64, t50956: f64, t8827: f64, t13776: f64, t3742: f64, t875: f64) -> (f64, f64, f64, f64) {
    let t56776 = t3959 * t9932;
    let t56782 = t4386 * t3897;
    let t56783 = t13792 * t56782;
    let t56787 = t3972 * t50956 * t8827 * t15167;
    let t56791 = t13776 * t50956 * t3742 * t875;
    (t56776, t56783, t56787, t56791)
}
