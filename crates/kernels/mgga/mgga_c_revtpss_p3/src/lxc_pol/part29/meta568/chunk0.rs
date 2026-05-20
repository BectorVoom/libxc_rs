//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1915/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1915<F: Float>(t14767: F, t27159: F, t4537: F, t605: F, t15071: F, t30: F, t25207: F, t61203: F, t4433: F, t892: F, t14749: F, t18875: F, t92790: F) -> (F, F, F, F, F, F, F) {
    let t98699 = t27159 * t14767;
    let t98702 = t605 * t4537;
    let t98705 = t30 * t15071;
    let t98709 = t25207 * t61203;
    let t98713 = t892 * t605 * t4433;
    let t98716 = t27159 * t14749;
    let t98733 = t92790 * t18875;
    (t98699, t98702, t98705, t98709, t98713, t98716, t98733)
}
