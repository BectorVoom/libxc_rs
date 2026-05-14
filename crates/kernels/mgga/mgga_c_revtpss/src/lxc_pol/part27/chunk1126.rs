//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1126/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1126<F: Float>(t10073: F, t25937: F, t7274: F, t7282: F, t1955: F, t9656: F, t1398: F, t4077: F, t543: F, t25904: F, t94634: F, t94640: F, t10146: F, t2022: F, t25921: F, t25924: F, t25931: F, t25966: F, t26034: F, t26036: F, t27868: F, t46433: F, t7292: F, t7295: F, t7296: F, t7301: F, t94799: F, t94803: F, t94807: F, t94811: F, t94813: F) -> (F,) {
    let t94820 = t10073 * t7282 * t25937 * t7274;
    let t94823 = t1955 * t7282 * t9656;
    let t94825 = t4077 * t1398 * t543;
    let t94842 = t25904 * t94634;
    let t94844 = t25904 * t94640;
    let t94846 = 0.13010442282307799193e1 * t25921 * t25966 - 0.13010442282307799193e1 * t7292 * t26036 - 0.29272321618148349057e-1 * t94799 + 0.77108554593144223218e-1 * t94803 + 0.51405703062096148814e-2 * t94807 + 0.21684070470512998656e-1 * t94811 + 0.15421710918628844643e0 * t94813 + 0.13010442282307799193e1 * t27868 * t25931 * t46433 - 0.72280234901709995519e-3 * t94820 + 0.78062653693846795158e1 * t94823 * t25931 * t94825 + 0.13010442282307799193e1 * t7295 * t7301 * t26034 * t1398 * t543 + 0.8673628188205199462e0 * t7295 * t7296 * t2022 * t10146 - 0.78062653693846795158e1 * t7295 * t25924 * t7274 * t4077 + 0.28912093960683998208e-1 * t94842 - 0.21684070470512998656e-1 * t94844;
    (t94846,)
}
