//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1199/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1199(t115027: f64, t115040: f64, t115052: f64, t115065: f64, t102101: f64, t102120: f64, t102122: f64, t109391: f64, t109393: f64, t109397: f64, t109400: f64, t109404: f64, t109408: f64, t109413: f64, t1903: f64, t2027: f64, t2028: f64, t25924: f64, t30296: f64, t30308: f64, t545: f64, t7295: f64, t7917: f64, t96206: f64) -> (f64, f64) {
    let t115067 = t115027 + t115040 + t115052 + t115065;
    let t115074 = 0.51405703062096148814e-2_f64 * t102101 + 0.86736281882051994623e-1_f64 * t109391 - 0.15421710918628844643e0_f64 * t109393 - 0.78062653693846795158e1_f64 * t7295 * t25924 * t30308 * t1903 - 0.43368140941025997312e-1_f64 * t109397 + 0.77108554593144223218e-1_f64 * t109400 + 0.13010442282307799194e0_f64 * t109404 + 0.21684070470512998656e-1_f64 * t109408 + 0.14456046980341999104e-2_f64 * t102120 - 0.86736281882051994623e-1_f64 * t109413 - 0.39029762157531132076e-1_f64 * t102122 + t96206 - 0.4336814094102599731e0_f64 * t2027 * t2028 * t545 * t115067 - 0.13010442282307799193e1_f64 * t7917 * t30296;
    (t115067, t115074)
}
