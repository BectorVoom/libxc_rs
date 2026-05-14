//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 695/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk695<F: Float>(t127: F, t6: F, t6867: F, t161: F, t2030: F, t2070: F, t105: F, t2156: F, t635: F, t2022: F, t645: F) -> (F, F, F, F, F, F) {
    let t6869 = t6 * t6867 * t127;
    let t6870 = t161 * t6869;
    let t6873 = t2030 * t2070;
    let t6875 = t105 * t2156;
    let t6876 = t6875 * t635;
    let t6877 = t2022 * t645;
    (t6869, t6870, t6873, t6875, t6876, t6877)
}
