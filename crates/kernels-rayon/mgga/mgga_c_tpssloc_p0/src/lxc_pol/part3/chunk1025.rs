//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1025/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1025(t210: f64, t4158: f64, t776: f64, t1495: f64, t2553: f64, t120: f64, t4119: f64, t2645: f64, t829: f64, t2679: f64, t4248: f64, t13242: f64, t4180: f64) -> (f64, f64, f64, f64, f64) {
    let t13293 = t210 * t4158 * t776;
    let t13297 = t210 * t1495 * t2553;
    let t13300 = t120 * t4119;
    let t13302 = t2645 * t13300 * t829;
    let t13306 = t2645 * t4248 * t2679;
    let t13312 = t4180 * t13242 * t829;
    (t13293, t13297, t13302, t13306, t13312)
}
