//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1380/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1380<F: Float>(t234: F, t9801: F, t10887: F, t136: F, t2475: F, t220: F, t10777: F, t2731: F, t837: F, t2482: F, t2668: F, t823: F) -> (F, F, F) {
    let t40721 = t9801 * t234;
    let t40722 = t40721 * t10887;
    let t40724 = t2475 * t136;
    let t40725 = t40724 * t220;
    let t40728 = t10777 * t40725 * t2731 * t837;
    let t40731 = t2482 * t823 * t2668;
    (t40722, t40728, t40731)
}
