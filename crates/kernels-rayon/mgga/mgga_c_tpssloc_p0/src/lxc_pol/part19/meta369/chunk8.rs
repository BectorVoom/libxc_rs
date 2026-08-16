//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1368/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1368(t11018: f64, t225: f64, t3206: f64, t11016: f64, t10160: f64, t10170: f64, t10182: f64, t10358: f64, t1049: f64, t1052: f64, t1066: f64, t11007: f64, t11010: f64, t11085: f64, t3020: f64, t3026: f64, t3166: f64, t3169: f64, t3174: f64, t3176: f64, t3207: f64, t349: f64, t388: f64, t43419: f64, t990: f64) -> f64 {
    let t43431 = t11018 * t225;
    let t43436 = t3206 * t3206;
    let t43440 = t11016 * t225;
    let t43447 = 4.0_f64 * t10358 * t1049 * t388 + 6.0_f64 * t1052 * t3174 * t43436 + 4.0_f64 * t11007 * t388 * t990 + 6.0_f64 * t3020 * t3166 * t388 + t349 * t388 * t43419 + 24.0_f64 * t10160 * t3176 + 12.0_f64 * t10170 * t3176 + 24.0_f64 * t10182 * t3026 + 24.0_f64 * t10182 * t3169 - 12.0_f64 * t1066 * t43431 - 4.0_f64 * t1066 * t43440 - 6.0_f64 * t11010 * t3207 - 4.0_f64 * t11085 * t3169;
    t43447
}
