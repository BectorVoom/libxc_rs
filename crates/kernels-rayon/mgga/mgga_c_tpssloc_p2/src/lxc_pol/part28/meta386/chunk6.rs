//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1510/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1510(t14687: f64, t15856: f64, t3701: f64, t5356: f64, t3719: f64, t5127: f64, t5168: f64, t588: f64, t592: f64, t5166: f64, t5187: f64, t571: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15857 = t14687 + t15856;
    let t15868 = t5356 * t3701;
    let t15872 = t5127 * t3719;
    let t15875 = t588 * t5168;
    let t15876 = 8.0_f64 * t15875;
    let t15877 = t592 * t5168;
    let t15878 = 8.0_f64 * t15877;
    let t15880 = 8.0_f64 * t588 * t5166;
    let t15883 = t571 * t5187;
    (t15857, t15868, t15872, t15876, t15878, t15880, t15883)
}
