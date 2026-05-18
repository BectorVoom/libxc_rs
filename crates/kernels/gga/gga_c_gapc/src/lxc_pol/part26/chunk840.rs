//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 840/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk840<F: Float>(t1087: F, t3406: F, t829: F, t9786: F, t3434: F, t954: F, t7204: F, t9645: F, t2706: F, t3103: F, t3397: F, t7073: F, t8673: F) -> (F, F, F, F, F, F) {
    let t9787 = t1087 * t3406;
    let t9788 = t829 * t9787;
    let t9789 = t9786 * t9788;
    let t9791 = t3434 * t954;
    let t9793 = t7204 * t9645;
    let t9795 = t2706 * t3103;
    let t9796 = t9795 * t3397;
    let t9798 = t7073 * t8673;
    (t9787, t9789, t9791, t9793, t9796, t9798)
}
