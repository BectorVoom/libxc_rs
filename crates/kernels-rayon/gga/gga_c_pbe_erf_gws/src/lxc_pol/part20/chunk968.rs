//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 968/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk968(t10917: f64, t1820: f64, t1627: f64, t3500: f64, t1648: f64, t3504: f64, t3522: f64, t5480: f64, t639: f64, t1630: f64, t3518: f64, t3512: f64, t5493: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10919 = 8.0_f64 / 15.0_f64 * t1820 * t10917;
    let t10921 = 8.0_f64 / 45.0_f64 * t1627 * t3500;
    let t10923 = 8.0_f64 / 45.0_f64 * t1648 * t3504;
    let t10924 = t5480 * t3522;
    let t10925 = t639 * t10924;
    let t10926 = 8.0_f64 / 81.0_f64 * t10925;
    let t10927 = t1630 * t3518;
    let t10928 = t639 * t10927;
    let t10929 = 8.0_f64 / 135.0_f64 * t10928;
    let t10930 = t5493 * t3512;
    (t10919, t10921, t10923, t10926, t10929, t10930)
}
