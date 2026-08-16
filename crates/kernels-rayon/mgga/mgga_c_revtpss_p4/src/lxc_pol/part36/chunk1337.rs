//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1337/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1337(t1903: f64, t6874: f64, t108332: f64, t108335: f64, t108337: f64, t25924: f64, t25930: f64, t25931: f64, t27868: f64, t6918: f64, t7295: f64, t7920: f64, t86641: f64, t94682: f64, t94700: f64, t94703: f64, t97894: f64, t97900: f64, t97917: f64, t97923: f64, t97926: f64, t97956: f64) -> f64 {
    let t114621 = t6874 * t1903;
    let t114632 = -0.78062653693846795158e1_f64 * t7295 * t25924 * t7920 * t6918 - 0.19514881078765566037e-2_f64 * t97894 + 0.28912093960683998208e-1_f64 * t97900 + t94682 + 0.51405703062096148814e-2_f64 * t97917 + 0.51405703062096148814e-2_f64 * t97923 - 0.28912093960683998208e-1_f64 * t97926 + t94700 - t94703 - 0.26020884564615598386e1_f64 * t25930 * t25931 * t114621 + 0.21684070470512998656e-1_f64 * t108332 - 0.43368140941025997312e-1_f64 * t108335 + 0.77108554593144223218e-1_f64 * t108337 + 0.72280234901709995519e-3_f64 * t97956 + 0.13010442282307799193e1_f64 * t27868 * t25931 * t86641;
    t114632
}
