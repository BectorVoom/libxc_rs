//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1255/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1255<F: Float>(t16901: F, t20347: F, t16919: F, t501: F, t8775: F, t8777: F, t16926: F, t20353: F, t20356: F, t20358: F, t16775: F, t16779: F, t16783: F, t16787: F, t16906: F, t16909: F, t16915: F, t16923: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24648 = 0.10389515463408878255e3 * t16901;
    let t24649 = 8.0 * t20347;
    let t24650 = 0.24415263074675393405e-3 * t16919;
    let t24651 = t501 * t8775;
    let t24652 = 8.0 * t24651;
    let t24653 = t501 * t8777;
    let t24654 = 8.0 * t24653;
    let t24655 = 480.0 * t16926;
    let t24656 = 0.20508037716432813315e4 * t20353;
    let t24657 = 0.11696447245269292414e1 * t20356;
    let t24658 = 0.23392894490538584828e1 * t20358;
    let t24659 = -t16775 - t16779 + t16783 - t16787 - t24648 - t16906 + t16909 - t24649 + t16915 + t24650 - t16923 - t24652 - t24654 - t24655 - t24656 - t24657 - t24658;
    (t24648, t24649, t24650, t24652, t24654, t24655, t24656, t24657, t24658, t24659)
}
