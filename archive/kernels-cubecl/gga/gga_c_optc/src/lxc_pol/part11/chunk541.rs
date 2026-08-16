//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 541/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk541<F: Float>(t1129: F, t1516: F, t1515: F, t309: F, t441: F) -> (F, F, F) {
    let t4366 = t1516 * t1129;
    let t4368 = t1515 * t309;
    let t4369 = t441 * t4368;
    (t4366, t4368, t4369)
}
