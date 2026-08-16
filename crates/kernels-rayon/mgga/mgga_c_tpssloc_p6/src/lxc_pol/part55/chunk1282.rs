//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1282/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1282(t24574: f64, t34288: f64, t477: f64, t8054: f64, t32454: f64, t7999: f64, t1090: f64, t118175: f64, t1215: f64, t1244: f64, t1246: f64, t1653: f64, t1716: f64, t2121: f64, t2147: f64, t24849: f64, t27406: f64, t27532: f64, t27721: f64, t32459: f64, t32469: f64, t32470: f64, t34277: f64, t34300: f64, t3610: f64, t462: f64, t4930: f64, t5068: f64, t7283: f64, t7327: f64, t7362: f64, t8082: f64, t8891: f64) -> f64 {
    let t125550 = t24574 * t34288;
    let t125558 = t477 * t8054;
    let t125563 = t7999 * t32454;
    let t125568 = 2.0_f64 * t3610 * t34300 * t5068 + 0.16449340668482264365e-1_f64 * t2121 * t462 * t2147 * t27721 - 0.54831135561607547883e-2_f64 * t24849 * t7327 * t8082 * t27532 - 0.43864908449286038307e-1_f64 * t7999 * t32470 - 0.16449340668482264365e-1_f64 * t7283 * t4930 * t8891 - 0.16449340668482264365e-1_f64 * t7283 * t1716 * t32469 - 0.54831135561607547883e-2_f64 * t125550 + 0.14621636149762012769e-1_f64 * t27406 * t32459 - 0.54831135561607547883e-2_f64 * t7283 * t7362 * t118175 * t1653 - 0.54831135561607547883e-2_f64 * t7283 * t7362 * t125558 * t1090 - 0.14621636149762012769e-1_f64 * t125563 + t1244 * t34277 * t1215 * t1246;
    t125568
}
