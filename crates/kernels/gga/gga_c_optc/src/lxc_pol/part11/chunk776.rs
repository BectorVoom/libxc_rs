//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 776/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk776<F: Float>(t4561: F, t6713: F, t4570: F, t6724: F, t2030: F, t4656: F, t4652: F, t4579: F, t591: F, t40: F, t1: F, t598: F) -> (F, F, F, F, F, F, F, F) {
    let t13064 = t6713 * t4561;
    let t13076 = t6724 * t4570;
    let t13092 = t2030 * t4656;
    let t13094 = t2030 * t4652;
    let t13110 = t4579 * t591;
    let t13111 = t40 * t13110;
    let t13113 = t4579 * t1;
    let t13114 = t13113 * t598;
    (t13064, t13076, t13092, t13094, t13110, t13111, t13113, t13114)
}
