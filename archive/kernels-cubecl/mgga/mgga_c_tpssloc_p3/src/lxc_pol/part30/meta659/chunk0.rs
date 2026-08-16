//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2079/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2079<F: Float>(t26211: F, t6883: F, t268: F, t557: F, t6559: F, t26333: F, t81326: F, t22642: F, t22643: F, t7700: F, t22674: F, t26202: F, t6897: F) -> (F, F, F, F, F) {
    let t90604 = t6883 * t26211;
    let t90605 = F::cast_from(0.38381794893125283518e-1_f64) * t90604;
    let t90607 = t6559 * t557 * t268;
    let t90609 = t90607 * t81326 * t26333;
    let t90642 = t22642 * t22643 * t7700;
    let t90645 = t6897 * t22674 * t26202;
    (t90605, t90607, t90609, t90642, t90645)
}
