//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1549/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1549<F: Float>(t3151: F, t3259: F, t12032: F, t359: F, t3043: F, t3298: F, t1024: F, t1082: F, t1087: F, t1089: F, t12122: F, t12132: F, t12133: F, t16410: F, t16520: F, t3075: F, t3118: F, t3133: F, t3153: F, t3204: F, t3287: F, t3299: F, t3304: F, t3305: F, t342: F, t378: F, t380: F, t42760: F, t42852: F, t42909: F, t43323: F, t999: F) -> (F, F) {
    let t43497 = t3259 * t3151;
    let t43504 = t359 * t12032;
    let t43512 = t3043 * t3298;
    let t43519 = -F::cast_from(0.26341796731742046395e1_f64) * t3287 * t42760 * t1089 + F::cast_from(0.15805078039045227836e2_f64) * t16410 * t12133 - F::cast_from(0.39512695097613069592e1_f64) * t3287 * t3075 * t3133 * t1089 + F::cast_from(0.15805078039045227836e2_f64) * t16520 * t12133 + F::cast_from(0.52683593463484092788e1_f64) * t3204 * t1082 * t42909 + F::cast_from(0.79025390195226139183e1_f64) * t3299 * t43497 * t3304 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t380 * t43323 - F::cast_from(0.26341796731742046395e1_f64) * t1024 * t43504 * t999 + F::cast_from(0.65854491829355115987e0_f64) * t1087 * t378 * t42852 * t1089 + F::cast_from(0.79025390195226139183e1_f64) * t43512 * t3305 - F::cast_from(0.15805078039045227836e2_f64) * t12122 * t3118 * t3153 * t12132;
    (t43497, t43519)
}
