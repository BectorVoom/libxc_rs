//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1261/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1261(t7063: f64, t860: f64, t25374: f64, t25378: f64, t10495: f64, t7053: f64, t93304: f64, t93306: f64, t93312: f64, t93315: f64, t93318: f64, t93322: f64, t93324: f64, t93326: f64, t93331: f64, t93334: f64, t93335: f64, t93337: f64, t93339: f64) -> (f64, f64) {
    let t93341 = t7063 * t860;
    let t93342 = t93341 * t25374;
    let t93343 = t93342 * t25378;
    let t93345 = -0.77108554593144223218e-1_f64 * t93304 + 0.51405703062096148812e-1_f64 * t93306 + 0.39512695097613069591e1_f64 * t7053 * t10495 + 0.77108554593144223218e-1_f64 * t93312 + 0.43368140941025997312e-1_f64 * t93315 - 0.23132566377943266966e0_f64 * t93318 - 0.43368140941025997312e-1_f64 * t93322 + 0.51405703062096148812e-1_f64 * t93324 - 0.43368140941025997312e-1_f64 * t93326 - 0.86736281882051994623e-1_f64 * t93331 - t93334 - 0.51405703062096148812e-1_f64 * t93335 - 0.21684070470512998656e-1_f64 * t93337 - 0.10281140612419229762e0_f64 * t93339 + 0.15421710918628844643e0_f64 * t93343;
    (t93341, t93345)
}
