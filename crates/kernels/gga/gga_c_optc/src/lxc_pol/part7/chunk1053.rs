//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1053/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1053<F: Float>(t162: F, t22858: F, t6792: F, t6799: F, t1948: F, t6785: F, t2034: F, t2037: F, t6893: F, t127: F, t616: F, t6877: F) -> (F, F, F, F, F, F) {
    let t22859 = t162 * t22858;
    let t22862 = t6799 * t6792;
    let t22864 = t6785 * t1948;
    let t22865 = t2034 * t22864;
    let t22868 = t6893 * t2037;
    let t22871 = t6877 * t127 * t616;
    (t22859, t22862, t22864, t22865, t22868, t22871)
}
