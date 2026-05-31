//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2815/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2815<F: Float>(t2439: F, t2912: F, t2915: F, t2909: F, t11821: F, t240: F, t2851: F, t25273: F, t268: F, t271: F) -> (F, F, F, F, F, F) {
    let t41281 = t2439 * t2912;
    let t41285 = t2439 * t2915;
    let t41287 = t2439 * t2909;
    let t41294 = t240 * t11821;
    let t41295 = t2851 * t2851;
    let t41296 = F::cast_from(1.0_f64) / t41295;
    let t41306 = t268 * t25273 * t271;
    (t41281, t41285, t41287, t41294, t41296, t41306)
}
