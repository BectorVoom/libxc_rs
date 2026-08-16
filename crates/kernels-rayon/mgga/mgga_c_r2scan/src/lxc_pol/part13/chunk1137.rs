//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1137/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1137(t2626: f64, t503: f64, t5119: f64, t2842: f64, t37699: f64, t1577: f64, t3308: f64, t8034: f64, t3295: f64, t7524: f64, t10760: f64, t25670: f64, t6093: f64) -> (f64, f64, f64, f64, f64) {
    let t39640 = t503 * t5119 * t2626;
    let t39642 = t37699 * t2842;
    let t39645 = t1577 * t3308 * t8034;
    let t39647 = t3295 * t7524;
    let t39650 = t6093 * t10760 * t25670;
    (t39640, t39642, t39645, t39647, t39650)
}
