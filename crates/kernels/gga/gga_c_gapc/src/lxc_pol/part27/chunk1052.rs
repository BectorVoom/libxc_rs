//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1052/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1052<F: Float>(t11380: F, t1448: F, t8788: F, t11509: F, t5626: F, t11513: F, t5392: F, t5395: F, t11514: F, t5633: F, t137: F, t1743: F, t190: F, t33235: F, t442: F, t5971: F) -> (F, F, F, F, F) {
    let t34987 = t11380 * t1448 * t8788;
    let t34989 = t11509 * t5626;
    let t34992 = t5395 * t11513 * t5392;
    let t34995 = t11514 * t5633;
    let t35001 = t1743 * t33235 * t5971 * t190 * t137 * t442;
    (t34987, t34989, t34992, t34995, t35001)
}
