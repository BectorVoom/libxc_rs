//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 892/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk892(t2376: f64, t3886: f64, t829: f64, t830: f64, t3912: f64, t4396: f64, t2358: f64, t2246: f64, t3903: f64, t1109: f64, t376: f64, t810: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9897 = t2376 * t3886;
    let t9899 = t829 * t830 * t9897;
    let t9902 = t3912 * t4396;
    let t9907 = t3912 * t2358;
    let t9912 = t2246 * t3903;
    let t9914 = t376 * t1109;
    let t9915 = t9914 * t810;
    (t9897, t9899, t9902, t9907, t9912, t9914, t9915)
}
