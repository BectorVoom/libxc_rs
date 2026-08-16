//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 705/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk705(t1830: f64, t695: f64, t5434: f64, t61: f64, t717: f64, t1719: f64, t749: f64, t1883: f64, t625: f64, t626: f64, t630: f64, t648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5435 = t1830 * t695;
    let t5437 = 0.3903689268108626343e0_f64 * t5434 * t5435;
    let t5438 = t61 * t717;
    let t5439 = t749 * t1719;
    let t5441 = 0.57791679765211885293e1_f64 * t5438 * t5439;
    let t5444 = 0.53424999999999999999e-1_f64 * t625 * t626 * t1883;
    let t5446 = 1.0_f64 / t648 / t630;
    (t5435, t5437, t5439, t5441, t5444, t5446)
}
