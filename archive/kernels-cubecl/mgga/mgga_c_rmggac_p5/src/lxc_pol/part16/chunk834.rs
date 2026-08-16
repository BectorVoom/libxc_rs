//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 834/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk834<F: Float>(t1635: F, t2084: F, t8746: F, t8761: F, t1624: F, t8764: F, t1627: F, t7599: F, t1632: F, t8750: F, t7603: F, t25607: F, t27: F) -> (F, F, F, F, F, F, F, F, F) {
    let t41296 = t2084 * t1635;
    let t41297 = t8746 * t41296;
    let t41299 = t8761 * t41296;
    let t41301 = t2084 * t1624;
    let t41302 = t8764 * t41301;
    let t41307 = t2084 * t1627;
    let t41308 = t7599 * t41307;
    let t41313 = t2084 * t1632;
    let t41314 = t7599 * t41313;
    let t41319 = t8750 * t41301;
    let t41323 = t7603 * t41307;
    let t41327 = t7603 * t41313;
    let t41329 = t25607 * t27;
    (t41297, t41299, t41302, t41308, t41314, t41319, t41323, t41327, t41329)
}
