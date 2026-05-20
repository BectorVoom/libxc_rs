//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1231/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1231<F: Float>(t7063: F, t860: F, t25374: F, t25378: F, t10495: F, t7053: F, t93304: F, t93306: F, t93312: F, t93315: F, t93318: F, t93322: F, t93324: F, t93326: F, t93331: F, t93334: F, t93335: F, t93337: F, t93339: F) -> (F, F) {
    let t93341 = t7063 * t860;
    let t93342 = t93341 * t25374;
    let t93343 = t93342 * t25378;
    let t93345 = -F::cast_from(0.77108554593144223218e-1_f64) * t93304 + F::cast_from(0.51405703062096148812e-1_f64) * t93306 + F::cast_from(0.39512695097613069591e1_f64) * t7053 * t10495 + F::cast_from(0.77108554593144223218e-1_f64) * t93312 + F::cast_from(0.43368140941025997312e-1_f64) * t93315 - F::cast_from(0.23132566377943266966e0_f64) * t93318 - F::cast_from(0.43368140941025997312e-1_f64) * t93322 + F::cast_from(0.51405703062096148812e-1_f64) * t93324 - F::cast_from(0.43368140941025997312e-1_f64) * t93326 - F::cast_from(0.86736281882051994623e-1_f64) * t93331 - t93334 - F::cast_from(0.51405703062096148812e-1_f64) * t93335 - F::cast_from(0.21684070470512998656e-1_f64) * t93337 - F::cast_from(0.10281140612419229762e0_f64) * t93339 + F::cast_from(0.15421710918628844643e0_f64) * t93343;
    (t93341, t93345)
}
