//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta182 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk832;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta182(t761: f64, t9892: f64, t2427: f64, t2655: f64, t152: f64, t31: f64, t185: f64, t9288: f64, t2448: f64, t67: f64, t758: f64, t2368: f64, t2505: f64, t745: f64, t9820: f64, t9824: f64, t9881: f64, t9884: f64, t9887: f64, t9890: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9894, t9896, t9897, t9898, t9900, t9901, t9903, t9905) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk832(t761, t9892, t2427, t2655, t152, t31, t185, t9288, t2448, t67, t758, t2368, t2505, t745);
        let (t9907, t9908) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk833(t761, t9905, t9820, t9824, t9881, t9884, t9887, t9890, t9894, t9896, t9900, t9903);
    (t9894, t9896, t9897, t9898, t9900, t9901, t9903, t9905, t9907, t9908)
}
