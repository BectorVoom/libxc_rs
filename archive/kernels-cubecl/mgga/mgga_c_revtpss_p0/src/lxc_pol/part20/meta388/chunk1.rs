//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1419/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1419<F: Float>(t11342: F, t698: F, t11821: F, t240: F, t2851: F, t39443: F, t141: F, t39457: F, t905: F, t930: F, t25273: F, t268: F, t271: F) -> (F, F, F, F, F, F, F) {
    let t41292 = t698 * t11342;
    let t41294 = t240 * t11821;
    let t41295 = t2851 * t2851;
    let t41296 = F::cast_from(1.0_f64) / t41295;
    let t41297 = t41296 * t39443;
    let t41299 = t141 * t41294 * t41297;
    let t41301 = t905 * t39457;
    let t41303 = t141 * t930 * t41301;
    let t41306 = t268 * t25273 * t271;
    (t41292, t41296, t41297, t41299, t41301, t41303, t41306)
}
