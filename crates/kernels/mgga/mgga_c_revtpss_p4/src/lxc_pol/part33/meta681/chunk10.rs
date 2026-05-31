//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2231/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2231<F: Float>(t109135: F, t109138: F, t109140: F, t109142: F, t109144: F, t109147: F, t109149: F, t109152: F, t109155: F, t109157: F, t109158: F, t109159: F, t109162: F, t109164: F, t109167: F, t109169: F, t111788: F, t111790: F, t1453: F, t18232: F, t30959: F, t569: F, t7586: F) -> F {
    let t111796 = (t111788 + t111790) * t569 + t30959 * t1453 + t109135 + t109138 + t109140 - t109142 - t109144 - t109147 - t109149 - t109152 - t109155 - t109157 - t109158 + t109159 - t109162 - t109164 - F::cast_from(2.0_f64) * t7586 * t18232 + t109167 - t109169;
    t111796
}
