//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3288/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3288<F: Float>(t40076: F, t40079: F, t40194: F, t40198: F, t62290: F, t62293: F, t62296: F, t62297: F, t62298: F, t62299: F, t62300: F, t62301: F, t62303: F, t62304: F, t62305: F, t62306: F, t62307: F, t62308: F, t62311: F, t62312: F) -> F {
    let t62313 = t62290 + t62293 + t62296 + t62297 + t62298 - t62299 + t62300 + t62301 + t62303 + t62304 + t62305 + t62306 + t40076 - t40079 + t40194 + t40198 + t62307 - t62308 + t62311 - t62312;
    t62313
}
