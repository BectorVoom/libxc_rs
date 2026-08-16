//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 837/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk837(t11218: f64, t28228: f64, t5192: f64, t6674: f64, t2364: f64, t9089: f64, t10365: f64, t5182: f64, t6719: f64, t8958: f64, t5054: f64, t2441: f64, t8672: f64) -> (f64, f64, f64, f64) {
    let t28242 = t11218 * t28228;
    let t28243 = t5192 * t28242;
    let t28244 = t6674 * t28243;
    let t28248 = t9089 * t2364;
    let t28249 = t10365 * t28248;
    let t28250 = t5182 * t28249;
    let t28252 = t6719 * t8958;
    let t28253 = t5054 * t28252;
    let t28256 = t8672 * t2441;
    (t28244, t28250, t28253, t28256)
}
