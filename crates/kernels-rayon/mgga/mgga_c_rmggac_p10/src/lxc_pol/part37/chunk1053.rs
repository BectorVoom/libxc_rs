//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1053/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1053(t68540: f64, t68543: f64, t68550: f64, t74321: f64, t74378: f64, t76968: f64, t76969: f64, t76970: f64, t76972: f64, t76973: f64, t76974: f64, t76975: f64, t76976: f64, t76977: f64, t76978: f64, t76979: f64, t76980: f64) -> f64 {
    let t80084 = 0.31062809106223861414e-2_f64 * t74321 - t76968 + t76969 - t76970 - t76972 + t76973 + t76974 + t68540 - t68543 + t76975 + t76976 - t76977 + t68550 + t76978 - t76979 - t76980 - 0.17519306092901367186e-5_f64 * t74378;
    t80084
}
