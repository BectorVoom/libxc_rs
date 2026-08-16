//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1065/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1065(t10856: f64, t5116: f64, t10707: f64, t1591: f64, t10710: f64, t20238: f64, t10810: f64, t1592: f64, t6166: f64, t10811: f64, t1584: f64, t5169: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37656 = t10856 * t5116;
    let t37658 = t1591 * t10707;
    let t37660 = t37658 * t10710 * t20238;
    let t37674 = t1592 * t10810 * t6166;
    let t37676 = t1584 * t10811;
    let t37681 = t10856 * t5169;
    (t37656, t37658, t37660, t37674, t37676, t37681)
}
