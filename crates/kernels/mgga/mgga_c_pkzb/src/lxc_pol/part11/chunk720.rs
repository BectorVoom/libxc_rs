//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 720/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk720<F: Float>(t5939: F, t762: F, t757: F, t749: F, t5717: F) -> (F, F, F, F) {
    let t5940 = t5939 * t762;
    let t5941 = t757 * t5940;
    let t5950 = t749 * t749;
    let t5951 = 1.0 / t5950;
    let t5952 = t5717 * t5951;
    (t5941, t5950, t5951, t5952)
}
