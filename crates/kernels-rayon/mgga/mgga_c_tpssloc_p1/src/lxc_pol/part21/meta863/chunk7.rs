//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3148/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3148(t19256: f64, t225: f64, t11606: f64, t11613: f64, t1190: f64, t1238: f64, t1252: f64, t15787: f64, t15794: f64, t15820: f64, t1761: f64, t19120: f64, t19214: f64, t19226: f64, t19232: f64, t3487: f64, t3593: f64, t3598: f64, t3599: f64, t3600: f64, t3630: f64, t491: f64, t4945: f64, t498: f64, t5055: f64, t5089: f64, t51937: f64, t52386: f64, t6243: f64, t6244: f64, t6267: f64, t65165: f64) -> f64 {
    let t65203 = t19256 * t225;
    let t65206 = -2.0_f64 * t52386 * t1761 + t65165 * t491 * t498 - 12.0_f64 * t3487 * t19226 - 6.0_f64 * t1238 * t11606 * t6243 * t3630 - 12.0_f64 * t3593 * t19226 - 2.0_f64 * t4945 * t15787 + 2.0_f64 * t19232 * t3600 + 2.0_f64 * t1190 * t19120 * t498 + 8.0_f64 * t3487 * t19214 + 4.0_f64 * t11613 * t6244 - 12.0_f64 * t5055 * t15794 - 4.0_f64 * t15820 * t5089 - 2.0_f64 * t51937 * t1761 - 6.0_f64 * t1238 * t11606 * t6267 * t3599 + 2.0_f64 * t1238 * t3598 * t6267 * t3630 + 8.0_f64 * t3593 * t19214 - 4.0_f64 * t65203 * t1252;
    t65206
}
