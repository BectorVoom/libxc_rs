//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2415/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2415<F: Float>(t11331: F, t698: F, t2439: F, t2912: F, t11328: F, t2915: F, t2909: F, t11345: F, t11342: F, t11821: F, t240: F, t2851: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41275 = t698 * t11331;
    let t41281 = t2439 * t2912;
    let t41283 = t698 * t11328;
    let t41285 = t2439 * t2915;
    let t41287 = t2439 * t2909;
    let t41289 = t698 * t11345;
    let t41292 = t698 * t11342;
    let t41294 = t240 * t11821;
    let t41295 = t2851 * t2851;
    (t41275, t41281, t41283, t41285, t41287, t41289, t41292, t41294, t41295)
}
