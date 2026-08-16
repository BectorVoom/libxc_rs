//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1104/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1104(t3261: f64, t5086: f64, t97: f64, t481: f64, t792: f64, t983: f64, t10609: f64, t1561: f64, t2625: f64, t11531: f64, t11584: f64, t37365: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39190 = t97 * t3261 * t5086;
    let t39192 = t983 * t481 * t792;
    let t39197 = t97 * t10609 * t1561;
    let t39198 = t2625 * t792;
    let t39209 = t11531 * t792;
    let t39215 = t37365 * t11584;
    (t39190, t39192, t39197, t39198, t39209, t39215)
}
