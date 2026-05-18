//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 812/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk812<F: Float>(t1218: F, t1573: F, t1217: F, t4280: F, t4535: F, t1583: F, t5255: F, t6: F, t1582: F) -> (F, F, F, F) {
    let t15072 = t1218 * t1573;
    let t15073 = t1217 * t15072;
    let t15078 = t4535 * t4280;
    let t15082 = t1583 * t5255 * t6;
    let t15083 = t1582 * t15082;
    (t15073, t15078, t15082, t15083)
}
