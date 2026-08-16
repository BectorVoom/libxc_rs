//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2336/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2336<F: Float>(t12879: F, t1715: F, t247: F, t1261: F, t16756: F, t5341: F, t3720: F, t12916: F, t5342: F, t5340: F, t12702: F, t5330: F) -> (F, F, F, F, F, F, F) {
    let t17416 = t247 * t12879 * t1715;
    let t17417 = t1261 * t17416;
    let t17419 = t16756 * t5341;
    let t17420 = t3720 * t17419;
    let t17423 = t12916 * t5342;
    let t17425 = F::cast_from(0.57165357490759649296e-3_f64) * t5340 * t17423;
    let t17426 = t12702 * t5330;
    (t17416, t17417, t17419, t17420, t17423, t17425, t17426)
}
