//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1019/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1019(t1070: f64, t9640: f64, t3629: f64, t8358: f64, t2928: f64, t6661: f64, t2938: f64, t1276: f64, t11032: f64, t11058: f64, t12230: f64, t12235: f64, t12238: f64, t12587: f64, t12589: f64) -> (f64, f64, f64) {
    let t12591 = t9640 * t1070;
    let t12593 = t8358 * t3629;
    let t12595 = t1070 * t2928;
    let t12596 = t6661 * t12595;
    let t12598 = t1070 * t2938;
    let t12599 = t1276 * t12598;
    let t12601 = -t11032 - t12230 - t12587 / 4.0_f64 + t12589 / 8.0_f64 - t12591 / 8.0_f64 + t12593 / 2.0_f64 + t12235 - 3.0_f64 / 4.0_f64 * t12596 - t12238 + t12599 / 4.0_f64 - t11058;
    (t12595, t12598, t12601)
}
