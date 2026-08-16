//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1161/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1161<F: Float>(t17239: F, t2367: F, t930: F, t17249: F, t2569: F, t10838: F, t16921: F, t4038: F, t17317: F, t993: F, t16901: F, t999: F) -> (F, F, F, F, F) {
    let t52154 = t930 * t2367 * t17239;
    let t52200 = t17249 * t2569;
    let t52241 = t4038 * t10838 * t16921;
    let t52245 = t17317 * t993;
    let t52260 = t999 * t2367 * t16901;
    (t52154, t52200, t52241, t52245, t52260)
}
