//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1076/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1076<F: Float>(t93020: F, t2482: F, t25260: F, t27: F, t596: F, t7036: F, t2681: F, t820: F, t843: F, t10867: F, t64: F, t239: F, t7043: F, t240: F, t233: F, t41077: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93021 = 0.14450132032386466905e-2 * t93020;
    let t93025 = t2482 * t25260 * t27;
    let t93034 = t2482 * t7036 * t596;
    let t93048 = t820 * t7036 * t2681;
    let t93054 = t820 * t25260 * t843;
    let t93060 = t10867 * t64;
    let t93062 = t820 * t93060 * t239;
    let t93066 = t820 * t7043 * t2681;
    let t93072 = t2482 * t7043 * t596;
    let t93082 = t25260 * t240;
    let t93118 = t41077 * t233;
    (t93021, t93025, t93034, t93048, t93054, t93062, t93066, t93072, t93082, t93118)
}
