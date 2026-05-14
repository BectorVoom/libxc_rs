//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1036/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1036<F: Float>(t26485: F, t93342: F, t10509: F, t26481: F, t25387: F, t11015: F, t7388: F, t212: F, t26473: F, t689: F, t780: F, t26474: F, t686: F, t72: F, t7058: F, t7064: F) -> (F, F, F, F, F, F, F) {
    let t95624 = t93342 * t26485;
    let t95628 = t26481 * t10509;
    let t95629 = t25387 * t95628;
    let t95632 = 0.30356481678079769392e-1 * t7388 * t11015;
    let t95635 = t689 * t212 * t26473 * t780;
    let t95644 = t26474 * t72 * t686;
    let t95645 = t7058 * t95644;
    let t95647 = t7064 * t95644;
    (t95624, t95628, t95629, t95632, t95635, t95645, t95647)
}
