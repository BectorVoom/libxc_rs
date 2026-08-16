//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 516/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk516<F: Float>(t799: F, t2118: F, t2019: F, t306: F, t2027: F, t2029: F, t272: F, t296: F) -> (F, F, F, F, F) {
    let t2119 = t799 * t799;
    let t2120 = t2118 * t2119;
    let t2123 = t2019 * t306;
    let t2124 = t2027 * t2029;
    let t2126 = F::cast_from(1.0_f64) / t296 / t272;
    (t2119, t2120, t2123, t2124, t2126)
}
