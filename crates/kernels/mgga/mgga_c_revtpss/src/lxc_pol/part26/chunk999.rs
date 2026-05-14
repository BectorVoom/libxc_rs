//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 999/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk999<F: Float>(t2677: F, t93072: F, t10737: F, t7045: F, t10741: F, t25234: F, t10709: F, t25227: F, t2661: F, t240: F, t25260: F, t10728: F, t2479: F, t25222: F, t25228: F, t9775: F) -> (F, F, F, F, F, F, F) {
    let t93073 = t93072 * t2677;
    let t93075 = t7045 * t10737;
    let t93077 = t25234 * t10741;
    let t93080 = t2661 * t25227 * t10709;
    let t93082 = t25260 * t240;
    let t93084 = t2661 * t93082 * t10728;
    let t93086 = t25222 * t2479;
    let t93088 = t9775 * t25228;
    (t93073, t93075, t93077, t93080, t93084, t93086, t93088)
}
