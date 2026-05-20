//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2242/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2242<F: Float>(t18245: F, t7003: F, t1518: F, t4245: F, t1937: F, t1501: F, t4292: F, t30138: F, t6993: F, t29506: F, t7316: F, t30112: F, t7235: F) -> (F, F, F, F, F, F, F, F) {
    let t109149 = F::new(2.0) * t18245 * t7003;
    let t109150 = t4245 * t1518;
    let t109152 = F::new(4.0) * t109150 * t1937;
    let t109153 = t1501 * t4292;
    let t109155 = F::new(4.0) * t109153 * t1937;
    let t109157 = F::new(4.0) * t30138 * t6993;
    let t109158 = t29506 * t7316;
    let t109159 = t7235 * t30112;
    (t109149, t109150, t109152, t109153, t109155, t109157, t109158, t109159)
}
