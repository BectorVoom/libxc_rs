//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 840/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk840(t2364: f64, t9029: f64, t5184: f64, t5182: f64, t2441: f64, t5193: f64, t7718: f64, t5192: f64, t6974: f64, t8947: f64, t1869: f64, t6719: f64, t8866: f64) -> (f64, f64, f64, f64) {
    let t28275 = t9029 * t2364;
    let t28276 = t5184 * t28275;
    let t28277 = t5182 * t28276;
    let t28280 = t5193 * t7718 * t2441;
    let t28281 = t5192 * t28280;
    let t28282 = t5182 * t28281;
    let t28284 = t6974 * t8947;
    let t28285 = t1869 * t28284;
    let t28287 = t6719 * t8866;
    (t28277, t28282, t28285, t28287)
}
