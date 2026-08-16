//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1336/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1336(t25904: f64, t94634: f64, t94640: f64, t10146: f64, t1398: f64, t2022: f64, t25921: f64, t25924: f64, t25931: f64, t25966: f64, t26034: f64, t26036: f64, t27868: f64, t4077: f64, t46433: f64, t543: f64, t7274: f64, t7292: f64, t7295: f64, t7296: f64, t7301: f64, t94799: f64, t94803: f64, t94807: f64, t94811: f64, t94813: f64, t94820: f64, t94823: f64, t94825: f64) -> f64 {
    let t94842 = t25904 * t94634;
    let t94844 = t25904 * t94640;
    let t94846 = 0.13010442282307799193e1_f64 * t25921 * t25966 - 0.13010442282307799193e1_f64 * t7292 * t26036 - 0.29272321618148349057e-1_f64 * t94799 + 0.77108554593144223218e-1_f64 * t94803 + 0.51405703062096148814e-2_f64 * t94807 + 0.21684070470512998656e-1_f64 * t94811 + 0.15421710918628844643e0_f64 * t94813 + 0.13010442282307799193e1_f64 * t27868 * t25931 * t46433 - 0.72280234901709995519e-3_f64 * t94820 + 0.78062653693846795158e1_f64 * t94823 * t25931 * t94825 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t26034 * t1398 * t543 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2022 * t10146 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t7274 * t4077 + 0.28912093960683998208e-1_f64 * t94842 - 0.21684070470512998656e-1_f64 * t94844;
    t94846
}
