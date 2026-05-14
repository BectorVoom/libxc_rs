//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 959/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk959<F: Float>(t3375: F, t5124: F, t4389: F, t5138: F, t1163: F, t1165: F, t1552: F, t322: F, t4437: F, t3431: F, t4979: F, t330: F, t5291: F, t3207: F, t509: F, t3382: F, t4316: F) -> (F, F, F, F, F, F, F) {
    let t18810 = t3375 * t5124;
    let t18812 = t4389 * t5138;
    let t18817 = t1163 * t1165 * t1552 * t4437 * t322;
    let t18819 = t3431 * t4979;
    let t18828 = t330 * t5291;
    let t18830 = t3207 * t509;
    let t18832 = t3382 * t4316;
    (t18810, t18812, t18817, t18819, t18828, t18830, t18832)
}
