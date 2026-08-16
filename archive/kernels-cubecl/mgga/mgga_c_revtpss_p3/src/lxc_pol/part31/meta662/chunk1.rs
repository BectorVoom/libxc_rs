//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2243/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2243<F: Float>(t27833: F, t7937: F, t28189: F, t7898: F, t29506: F, t7239: F, t2014: F, t30111: F, t7315: F, t109135: F, t109138: F, t109140: F, t109142: F, t109144: F, t109147: F, t109149: F, t109152: F, t109155: F, t109157: F, t109158: F, t109159: F, t2011: F, t22506: F, t5787: F, t6934: F, t7231: F, t7894: F) -> F {
    let t109162 = F::cast_from(2.0_f64) * t27833 * t7937;
    let t109164 = F::cast_from(2.0_f64) * t7898 * t28189;
    let t109167 = F::cast_from(3.0_f64) * t29506 * t7239;
    let t109169 = t2014 * t30111 * t7315;
    let t109170 = t2011 * t22506 + F::cast_from(2.0_f64) * t5787 * t7894 + t6934 * t7231 + t109135 + t109138 + t109140 - t109142 - t109144 - t109147 - t109149 - t109152 - t109155 - t109157 - t109158 + t109159 - t109162 - t109164 + t109167 - t109169;
    t109170
}
