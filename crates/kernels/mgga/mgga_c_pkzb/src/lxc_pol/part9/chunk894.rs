//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 894/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk894<F: Float>(t1107: F, t5493: F, t1956: F, t1084: F, t1856: F, t1899: F, t1100: F, t1976: F, t1088: F, t1937: F, t1079: F, t1878: F, t218: F, t204: F, t2739: F, t648: F) -> (F, F, F, F, F, F, F, F) {
    let t7308 = t1107 * t5493;
    let t7309 = t7308 * t1956;
    let t7312 = t1084 * t1856;
    let t7314 = 6.0 * t1899 * t7312;
    let t7315 = t1100 * t1976;
    let t7324 = t1088 * t1937;
    let t7332 = t218 * t1878 * t1079;
    let t7335 = t204 * t648 * t2739;
    (t7308, t7309, t7312, t7314, t7315, t7324, t7332, t7335)
}
