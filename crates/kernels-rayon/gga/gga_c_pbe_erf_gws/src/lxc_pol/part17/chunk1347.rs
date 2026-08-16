//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1347/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1347(t13781: f64, t13782: f64, t3038: f64, t3972: f64, t1192: f64, t26654: f64, t829: f64, t830: f64, t13792: f64, t8716: f64, t13808: f64, t14584: f64) -> (f64, f64, f64, f64) {
    let t54707 = t3972 * t13781 * t3038 * t13782;
    let t54709 = t26654 * t1192;
    let t54711 = t829 * t830 * t54709;
    let t54714 = t13792 * t8716;
    let t54716 = t13808 * t14584;
    (t54707, t54711, t54714, t54716)
}
