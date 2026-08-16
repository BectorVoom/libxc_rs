//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1907/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1907<F: Float>(t268: F, t557: F, t6559: F, t26333: F, t81326: F, t22633: F, t26338: F, t80650: F, t1985: F, t22934: F, t26193: F, t16413: F, t214: F, t225: F, t567: F) -> (F, F, F, F, F) {
    let t90607 = t6559 * t557 * t268;
    let t90609 = t90607 * t81326 * t26333;
    let t90612 = t22633 * t80650 * t26338;
    let t90615 = t1985 * t26193 * t22934;
    let t90626 = t1985 * t214 * t16413 * t225 * t567;
    (t90607, t90609, t90612, t90615, t90626)
}
