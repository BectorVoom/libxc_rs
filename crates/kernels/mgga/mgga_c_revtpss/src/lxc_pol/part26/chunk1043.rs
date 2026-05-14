//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1043/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1043<F: Float>(t10510: F, t26497: F, t10073: F, t25402: F, t7056: F, t7398: F, t26481: F, t93182: F, t25411: F, t2754: F, t676: F, t136: F, t2457: F, t7423: F, t25299: F, t25431: F) -> (F, F, F, F, F, F, F, F) {
    let t95779 = t26497 * t10510;
    let t95783 = t10073 * t7056 * t25402 * t7398;
    let t95785 = t26481 * t93182;
    let t95786 = t25411 * t95785;
    let t95789 = t26481 * t676 * t2754;
    let t95790 = t25411 * t95789;
    let t95793 = t7423 * t136 * t2457;
    let t95794 = t25299 * t95793;
    let t95796 = t25431 * t95785;
    (t95779, t95783, t95786, t95789, t95790, t95793, t95794, t95796)
}
