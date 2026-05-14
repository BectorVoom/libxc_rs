//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 992/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk992<F: Float>(t10799: F, t27261: F, t10773: F, t25270: F, t10766: F, t10794: F, t7036: F, t820: F, t844: F, t2751: F, t2482: F, t814: F, t10782: F, t10803: F, t10807: F, t10744: F, t2664: F, t7028: F) -> (F, F, F, F, F, F, F, F, F) {
    let t92942 = t27261 * t10799;
    let t92944 = t25270 * t10773;
    let t92946 = t25270 * t10766;
    let t92948 = t25270 * t10794;
    let t92951 = t820 * t7036 * t844;
    let t92952 = t92951 * t2751;
    let t92955 = t2482 * t7036 * t814;
    let t92956 = t92955 * t10782;
    let t92958 = t25270 * t10803;
    let t92960 = t25270 * t10807;
    let t92963 = t10744 * t7028 * t2664;
    (t92942, t92944, t92946, t92948, t92952, t92956, t92958, t92960, t92963)
}
