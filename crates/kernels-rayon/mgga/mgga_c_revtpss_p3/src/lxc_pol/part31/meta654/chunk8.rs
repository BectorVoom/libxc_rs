//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2194/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2194(t1444: f64, t6844: f64, t30095: f64, t689: f64, t25904: f64, t25899: f64, t1903: f64, t543: f64, t5658: f64, t14224: f64, t1882: f64, t25930: f64, t25931: f64, t27837: f64, t27846: f64, t27868: f64, t27960: f64, t30055: f64, t30105: f64, t7295: f64, t7296: f64, t7301: f64, t94635: f64, t94648: f64, t94716: f64, t97823: f64, t97825: f64, t97838: f64, t97875: f64) -> f64 {
    let t108244 = t6844 * t1444;
    let t108248 = t30095 * t689;
    let t108249 = t25904 * t108248;
    let t108251 = t25899 * t108248;
    let t108259 = t1903 * t5658 * t543;
    let t108270 = -0.17347256376410398924e1_f64 * t25930 * t94716 * t30105 - 0.14634331517634470219e-1_f64 * t97823 + 0.26019841438354088051e-1_f64 * t97825 + 0.8673628188205199462e0_f64 * t27868 * t97875 * t14224 - 0.8673628188205199462e0_f64 * t25930 * t25931 * t108244 - 0.72280234901709995518e-2_f64 * t108249 + 0.12851425765524037203e-1_f64 * t108251 - 0.17135234354032049604e-1_f64 * t94635 + t94648 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t30055 * t1444 + t97838 - 0.17347256376410398924e1_f64 * t25930 * t25931 * t108259 + 0.8673628188205199462e0_f64 * t7295 * t7301 * t27960 * t1882 * t543 + 0.8673628188205199462e0_f64 * t27837 * t27846;
    t108270
}
