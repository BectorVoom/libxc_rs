//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1122/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1122<F: Float>(t108138: F, t94649: F, t27989: F, t98380: F, t689: F, t6919: F, t7242: F, t1364: F, t30074: F, t786: F, t30020: F, t686: F, t72: F, t25895: F, t30095: F, t25904: F) -> (F, F, F, F, F, F, F, F) {
    let t108141 = t94649 * t108138;
    let t108153 = t98380 * t27989;
    let t108156 = t689 * t7242 * t6919;
    let t108175 = t786 * t30074 * t1364;
    let t108187 = t30020 * t72 * t686;
    let t108188 = t25895 * t108187;
    let t108248 = t30095 * t689;
    let t108249 = t25904 * t108248;
    (t108141, t108153, t108156, t108175, t108187, t108188, t108248, t108249)
}
