//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 901/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk901(t1776: f64, t28312: f64, t1775: f64, t10833: f64, t28368: f64, t10832: f64, t7262: f64, t8820: f64, t7261: f64, t2364: f64, t5015: f64, t28385: f64, t7242: f64) -> (f64, f64, f64, f64, f64) {
    let t29010 = t1776 * t28312;
    let t29011 = t1775 * t29010;
    let t29016 = t10833 * t28368;
    let t29017 = t10832 * t29016;
    let t29024 = t7262 * t8820;
    let t29025 = t7261 * t29024;
    let t29028 = t2364 * t8820;
    let t29029 = t5015 * t29028;
    let t29032 = t7242 * t28385;
    (t29011, t29017, t29025, t29029, t29032)
}
