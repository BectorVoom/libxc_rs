//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1116/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1116<F: Float>(t10867: F, t64: F, t239: F, t820: F, t10874: F, t2681: F, t7043: F, t857: F, t25222: F, t2656: F, t2482: F, t596: F, t2677: F, t10737: F, t7045: F, t10741: F, t25234: F) -> (F, F, F, F, F, F) {
    let t93060 = t10867 * t64;
    let t93062 = t820 * t93060 * t239;
    let t93063 = t93062 * t10874;
    let t93066 = t820 * t7043 * t2681;
    let t93067 = t93066 * t857;
    let t93069 = t25222 * t2656;
    let t93072 = t2482 * t7043 * t596;
    let t93073 = t93072 * t2677;
    let t93075 = t7045 * t10737;
    let t93077 = t25234 * t10741;
    (t93063, t93067, t93069, t93073, t93075, t93077)
}
