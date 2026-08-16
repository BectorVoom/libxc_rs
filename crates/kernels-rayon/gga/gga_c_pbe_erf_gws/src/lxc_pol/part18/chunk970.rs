//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 970/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk970(t2581: f64, t7130: f64, t2567: f64, t2615: f64, t2579: f64, t34: f64, t7694: f64, t1820: f64, t1648: f64, t3415: f64, t10907: f64, t10912: f64, t10915: f64, t10919: f64, t10921: f64, t10923: f64, t10926: f64, t10929: f64, t10932: f64, t10934: f64, t10937: f64, t10942: f64, t7784: f64) -> (f64, f64, f64, f64, f64) {
    let t10944 = 16.0_f64 / 45.0_f64 * t7130 * t2581;
    let t10946 = 8.0_f64 / 45.0_f64 * t2615 * t2567;
    let t10947 = t2579 * t34;
    let t10948 = t7694 * t10947;
    let t10950 = 32.0_f64 / 45.0_f64 * t1820 * t10948;
    let t10952 = 8.0_f64 / 45.0_f64 * t1648 * t3415;
    let t10953 = t10907 - t10912 - t10915 - t10919 - t10921 - t10923 + t10926 + t10929 - t10932 + t10934 - t7784 - t10937 - t10942 + t10944 - t10946 + t10950 - t10952;
    (t10944, t10946, t10950, t10952, t10953)
}
