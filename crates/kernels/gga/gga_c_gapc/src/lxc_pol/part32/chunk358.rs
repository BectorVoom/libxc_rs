//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 358/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk358<F: Float>(t1672: F, t505: F, t1671: F, t632: F, t668: F, t457: F, t1665: F, t604: F, t624: F, t189: F, t190: F, t195: F, t633: F, t103: F, t198: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1673 = t1672 * t505;
    let t1674 = t1671 * t1673;
    let t1677 = t632 * t668;
    let t1678 = t1672 * t457;
    let t1679 = t1665 * t1678;
    let t1682 = t604 * t624;
    let t1686 = t189 * t190 * t195;
    let t1687 = t633 * t1686;
    let t1688 = t103 * t198;
    (t1673, t1674, t1677, t1678, t1679, t1682, t1686, t1687, t1688)
}
