//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1199/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1199<F: Float>(t115027: F, t115040: F, t115052: F, t115065: F, t102101: F, t102120: F, t102122: F, t109391: F, t109393: F, t109397: F, t109400: F, t109404: F, t109408: F, t109413: F, t1903: F, t2027: F, t2028: F, t25924: F, t30296: F, t30308: F, t545: F, t7295: F, t7917: F, t96206: F) -> (F, F) {
    let t115067 = t115027 + t115040 + t115052 + t115065;
    let t115074 = F::cast_from(0.51405703062096148814e-2_f64) * t102101 + F::cast_from(0.86736281882051994623e-1_f64) * t109391 - F::cast_from(0.15421710918628844643e0_f64) * t109393 - F::cast_from(0.78062653693846795158e1_f64) * t7295 * t25924 * t30308 * t1903 - F::cast_from(0.43368140941025997312e-1_f64) * t109397 + F::cast_from(0.77108554593144223218e-1_f64) * t109400 + F::cast_from(0.13010442282307799194e0_f64) * t109404 + F::cast_from(0.21684070470512998656e-1_f64) * t109408 + F::cast_from(0.14456046980341999104e-2_f64) * t102120 - F::cast_from(0.86736281882051994623e-1_f64) * t109413 - F::cast_from(0.39029762157531132076e-1_f64) * t102122 + t96206 - F::cast_from(0.4336814094102599731e0_f64) * t2027 * t2028 * t545 * t115067 - F::cast_from(0.13010442282307799193e1_f64) * t7917 * t30296;
    (t115067, t115074)
}
