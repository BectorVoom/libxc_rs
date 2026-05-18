//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1079/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1079<F: Float>(t23109: F, t4631: F, t23219: F, t4685: F, t4661: F, t7061: F, t23027: F, t4681: F, t4690: F, t7030: F, t23071: F, t4706: F) -> (F, F, F, F, F, F) {
    let t37600 = t23109 * t4631;
    let t37853 = t23219 * t4685;
    let t38033 = t7061 * t4661;
    let t38074 = t23027 * t4681;
    let t38105 = t7030 * t4690;
    let t38107 = t23071 * t4706;
    (t37600, t37853, t38033, t38074, t38105, t38107)
}
