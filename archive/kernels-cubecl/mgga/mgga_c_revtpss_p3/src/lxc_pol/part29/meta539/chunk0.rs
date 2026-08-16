//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1871/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1871<F: Float>(t2453: F, t26496: F, t10506: F, t10510: F, t26497: F, t10073: F, t25402: F, t7056: F, t7398: F, t26481: F, t93182: F, t25411: F) -> (F, F, F, F, F, F) {
    let t95773 = t2453 * t26496;
    let t95774 = t95773 * t10506;
    let t95779 = t26497 * t10510;
    let t95783 = t10073 * t7056 * t25402 * t7398;
    let t95785 = t26481 * t93182;
    let t95786 = t25411 * t95785;
    (t95773, t95774, t95779, t95783, t95785, t95786)
}
