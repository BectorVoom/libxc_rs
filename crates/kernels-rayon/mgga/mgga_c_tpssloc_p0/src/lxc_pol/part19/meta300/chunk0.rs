//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1084/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1084(t1229: f64, t3242: f64, t3493: f64, t3508: f64, t11153: f64, t3584: f64, t1089: f64, t1215: f64, t607: f64, t475: f64, t1332: f64, t5343: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15615 = t1229 * t3242;
    let t15620 = t3508 * t3493;
    let t15654 = t3584 * t11153;
    let t15660 = t1215 * t1089;
    let t15661 = t15660 * t607;
    let t15707 = t607 * t1215;
    let t15708 = t15707 * t475;
    let t16033 = t1332 * t5343;
    (t15615, t15620, t15654, t15661, t15708, t16033)
}
