//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 934/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk934<F: Float>(t1107: F, t5493: F, t1956: F, t1084: F, t1856: F, t1899: F, t1100: F, t1976: F) -> (F, F, F, F, F) {
    let t7308 = t1107 * t5493;
    let t7309 = t7308 * t1956;
    let t7312 = t1084 * t1856;
    let t7314 = 6.0 * t1899 * t7312;
    let t7315 = t1100 * t1976;
    (t7308, t7309, t7312, t7314, t7315)
}
