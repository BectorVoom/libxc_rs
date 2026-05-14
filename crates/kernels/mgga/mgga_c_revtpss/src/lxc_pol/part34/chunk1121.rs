//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1121/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1121<F: Float>(t29807: F, t994: F, t1647: F, t7810: F, t1078: F, t1982: F, t3140: F, t6343: F, t29894: F, t3336: F, t30088: F, t689: F, t25904: F, t25899: F, t30105: F, t94395: F) -> (F, F, F, F, F, F, F, F) {
    let t107566 = t994 * t29807;
    let t107629 = t1647 * t7810;
    let t107636 = t1982 * t6343 * t3140 * t1078;
    let t107741 = t29894 * t3336;
    let t108132 = t30088 * t689;
    let t108133 = t25904 * t108132;
    let t108135 = t25899 * t108132;
    let t108138 = t30105 * t689;
    let t108139 = t94395 * t108138;
    (t107566, t107629, t107636, t107741, t108133, t108135, t108138, t108139)
}
