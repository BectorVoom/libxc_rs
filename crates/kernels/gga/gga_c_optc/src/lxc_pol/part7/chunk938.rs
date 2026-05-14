//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 938/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk938<F: Float>(t2034: F, t22187: F, t2022: F, t6879: F, t2067: F, t162: F, t2024: F, t616: F, t6877: F, t127: F, t645: F, t6867: F, t2030: F, t6928: F, t6778: F, t6787: F, t6799: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t22188 = t2034 * t22187;
    let t22191 = t2022 * t6879;
    let t22192 = t22191 * t2067;
    let t22193 = t162 * t22192;
    let t22197 = t6877 * t2024 * t616;
    let t22198 = t2034 * t22197;
    let t22202 = t6867 * t645 * t127;
    let t22203 = t162 * t22202;
    let t22206 = t2030 * t6928;
    let t22208 = t2030 * t6778;
    let t22211 = t6867 * t2024 * t645;
    let t22212 = t162 * t22211;
    let t22215 = t6799 * t6787;
    (t22188, t22192, t22193, t22197, t22198, t22202, t22203, t22206, t22208, t22211, t22212, t22215)
}
