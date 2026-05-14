//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 852/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk852<F: Float>(t17022: F, t17078: F, t1411: F, t4990: F, t3861: F, t5049: F, t17064: F, t914: F, t17060: F, t3813: F, t4961: F, t3885: F, t16988: F, t288: F, t8197: F, t8210: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17079 = t17022 + t17078;
    let t17092 = t4990 * t1411;
    let t17096 = t3861 * t5049;
    let t17106 = t914 * t17064;
    let t17109 = t914 * t17060;
    let t17114 = t3813 * t4961;
    let t17115 = t3885 * t17114;
    let t17118 = t288 * t16988;
    let t17119 = t17118 * t8197;
    let t17122 = t17118 * t8210;
    (t17079, t17092, t17096, t17106, t17109, t17114, t17115, t17118, t17119, t17122)
}
