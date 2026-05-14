//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 721/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk721<F: Float>(t2002: F, t6: F, t4: F, t133: F, t5: F, t21: F, t1339: F, t1782: F) -> (F, F, F, F) {
    let t10194 = t6 * t2002;
    let t10195 = t4 * t10194;
    let t10344 = t5 * t133;
    let t10345 = t21 * t10344;
    let t10348 = t1782 * t1339;
    (t10194, t10195, t10345, t10348)
}
