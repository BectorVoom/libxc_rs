//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2231/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2231(t109135: f64, t109138: f64, t109140: f64, t109142: f64, t109144: f64, t109147: f64, t109149: f64, t109152: f64, t109155: f64, t109157: f64, t109158: f64, t109159: f64, t109162: f64, t109164: f64, t109167: f64, t109169: f64, t111788: f64, t111790: f64, t1453: f64, t18232: f64, t30959: f64, t569: f64, t7586: f64) -> f64 {
    let t111796 = (t111788 + t111790) * t569 + t30959 * t1453 + t109135 + t109138 + t109140 - t109142 - t109144 - t109147 - t109149 - t109152 - t109155 - t109157 - t109158 + t109159 - t109162 - t109164 - 2.0_f64 * t7586 * t18232 + t109167 - t109169;
    t111796
}
