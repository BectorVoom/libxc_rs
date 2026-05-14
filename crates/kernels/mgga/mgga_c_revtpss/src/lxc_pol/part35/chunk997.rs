//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 997/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk997<F: Float>(t2482: F, t596: F, t7043: F, t240: F, t25260: F, t233: F, t41077: F, t7056: F, t9646: F, t1954: F, t39643: F, t2453: F, t251: F, t25410: F, t25304: F, t25374: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93072 = t2482 * t7043 * t596;
    let t93082 = t25260 * t240;
    let t93118 = t41077 * t233;
    let t93134 = t9646 * t7056;
    let t93139 = t1954 * t39643;
    let t93140 = t93139 * t7056;
    let t93169 = t2453 * t251;
    let t93170 = t93169 * t25410;
    let t93189 = t25304 * t251;
    let t93190 = t93189 * t25374;
    (t93072, t93082, t93118, t93134, t93139, t93140, t93169, t93170, t93189, t93190)
}
