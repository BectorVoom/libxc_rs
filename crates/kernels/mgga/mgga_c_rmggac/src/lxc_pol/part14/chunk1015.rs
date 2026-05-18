//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1015/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1015<F: Float>(t25529: F, t36: F, t5169: F, t41027: F, t851: F, t2118: F, t41032: F, t22: F, t2353: F, t26531: F, t5184: F, t649: F) -> (F, F, F, F, F) {
    let t41262 = t25529 * t36;
    let t41263 = t41262 * t5169;
    let t41265 = t851 * t41027;
    let t41271 = t2118 * t41032;
    let t41274 = t26531 * t22 * t2353;
    let t41276 = t649 * t5184;
    (t41263, t41265, t41271, t41274, t41276)
}
