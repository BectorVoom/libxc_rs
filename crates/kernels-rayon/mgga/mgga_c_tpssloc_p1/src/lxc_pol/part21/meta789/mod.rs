//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta789 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2747;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta789(t17109: f64, t870: f64, t46206: f64, t12939: f64, t16716: f64, t2250: f64, t16558: f64, t184: f64, t4194: f64, t607: f64, t16619: f64, t16689: f64, t2430: f64, t12971: f64, t2522: f64, t39397: f64, t39400: f64, t39408: f64, t39411: f64, t40708: f64, t4310: f64, t4314: f64, t4315: f64, t776: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t57932, t57936, t57939, t57943, t57946, t57947) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2747(t17109, t870, t46206, t12939, t16716, t2250, t16558, t184, t4194, t607, t16619, t16689, t2430);
        let (t57948, t57955) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2748(t57947, t12971, t2522, t39397, t39400, t39408, t39411, t40708, t4310, t4314, t4315, t57932, t57936, t57939, t57943, t57946, t776);
    (t57936, t57939, t57943, t57946, t57948, t57955)
}
