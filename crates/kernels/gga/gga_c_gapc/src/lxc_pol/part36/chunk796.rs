//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 796/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk796<F: Float>(t9507: F, t9508: F, t8700: F, t889: F, t3397: F, t1068: F, t2387: F, t322: F, t3307: F, t913: F, t3288: F, t7577: F) -> (F, F, F, F, F) {
    let t9509 = t9507 * t9508;
    let t9512 = t889 * t8700;
    let t9513 = t9512 * t3397;
    let t9515 = t2387 * t1068;
    let t9516 = t9515 * t322;
    let t9518 = t3307 * t913;
    let t9520 = t3288 * t7577;
    (t9509, t9513, t9516, t9518, t9520)
}
