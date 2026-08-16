//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1549/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1549(t3151: f64, t3259: f64, t12032: f64, t359: f64, t3043: f64, t3298: f64, t1024: f64, t1082: f64, t1087: f64, t1089: f64, t12122: f64, t12132: f64, t12133: f64, t16410: f64, t16520: f64, t3075: f64, t3118: f64, t3133: f64, t3153: f64, t3204: f64, t3287: f64, t3299: f64, t3304: f64, t3305: f64, t342: f64, t378: f64, t380: f64, t42760: f64, t42852: f64, t42909: f64, t43323: f64, t999: f64) -> (f64, f64) {
    let t43497 = t3259 * t3151;
    let t43504 = t359 * t12032;
    let t43512 = t3043 * t3298;
    let t43519 = -0.26341796731742046395e1_f64 * t3287 * t42760 * t1089 + 0.15805078039045227836e2_f64 * t16410 * t12133 - 0.39512695097613069592e1_f64 * t3287 * t3075 * t3133 * t1089 + 0.15805078039045227836e2_f64 * t16520 * t12133 + 0.52683593463484092788e1_f64 * t3204 * t1082 * t42909 + 0.79025390195226139183e1_f64 * t3299 * t43497 * t3304 + 0.65854491829355115987e0_f64 * t342 * t380 * t43323 - 0.26341796731742046395e1_f64 * t1024 * t43504 * t999 + 0.65854491829355115987e0_f64 * t1087 * t378 * t42852 * t1089 + 0.79025390195226139183e1_f64 * t43512 * t3305 - 0.15805078039045227836e2_f64 * t12122 * t3118 * t3153 * t12132;
    (t43497, t43519)
}
