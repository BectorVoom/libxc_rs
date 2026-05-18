//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1222/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1222<F: Float>(t2595: F, t9: F, t2263: F, t2640: F, t7484: F, t2270: F, t3813: F, t7899: F, t889: F, t2613: F, t2620: F, t24985: F, t329: F) -> (F, F, F, F, F) {
    let t25217 = t9 * t2595;
    let t25218 = t25217 * t2263;
    let t25220 = t2640 * t25218 * t7484;
    let t25227 = t3813 * t2270;
    let t25237 = t7899 * t889;
    let t25239 = t2613 * t2620;
    let t25243 = t329 * t24985;
    (t25220, t25227, t25237, t25239, t25243)
}
