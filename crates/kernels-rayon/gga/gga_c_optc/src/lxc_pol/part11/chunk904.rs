//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 904/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk904(t16636: f64, t3836: f64, t16644: f64, t2813: f64, t14300: f64, t14339: f64, t1325: f64, t8201: f64, t16672: f64, t241: f64, t16824: f64, t16826: f64, t16885: f64, t16931: f64, t16935: f64, t16941: f64, t16945: f64, t16947: f64, t16955: f64, t16957: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17024 = t3836 * t16636;
    let t17028 = t2813 * t16644;
    let t17031 = t14300 * t14339;
    let t17034 = t8201 * t1325;
    let t17035 = t14300 * t17034;
    let t17039 = 0.19751789702565206229e-1_f64 * t241 * t16672;
    let t17040 = t16931 - t16935 - t16945 - t16941 - t16955 - t16957 + t16885 + t16947 + t17039 + t16824 + t16826;
    (t17024, t17028, t17031, t17034, t17035, t17039, t17040)
}
