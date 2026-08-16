//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 688/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk688<F: Float>(t640: F, t7353: F, t14090: F, t4765: F, t14224: F, t7254: F, t1179: F, t384: F, t1966: F, t1968: F, t14030: F, t14123: F, t68438: F) -> (F, F, F, F, F, F) {
    let t68949 = t640 * t7353;
    let t68950 = t4765 * t14090 * t68949;
    let t68990 = t7254 * t14224;
    let t69002 = t1179 * t384;
    let t69004 = t1966 * t69002 * t1968;
    let t69009 = t14030 * t68438 * t14123;
    (t68949, t68950, t68990, t69002, t69004, t69009)
}
