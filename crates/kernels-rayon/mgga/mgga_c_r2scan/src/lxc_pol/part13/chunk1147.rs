//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1147/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1147(t27182: f64, t3308: f64, t6449: f64, t10810: f64, t1592: f64, t8156: f64, t25813: f64, t6218: f64, t24064: f64, t5136: f64, t10743: f64, t2699: f64) -> (f64, f64, f64, f64, f64) {
    let t39759 = t6449 * t3308 * t27182;
    let t39762 = t1592 * t10810 * t8156;
    let t39763 = 0.69345773920434148506e0_f64 * t39762;
    let t39765 = t6218 * t3308 * t25813;
    let t39768 = t5136 * t3308 * t24064;
    let t39770 = t10743 * t2699;
    (t39759, t39763, t39765, t39768, t39770)
}
