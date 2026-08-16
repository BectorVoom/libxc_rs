//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2403/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2403(t41959: f64, t41962: f64, t47781: f64, t47785: f64, t47787: f64, t49043: f64, t49049: f64, t49052: f64, t49054: f64, t49056: f64, t49058: f64, t49060: f64) -> f64 {
    let t49219 = 0.58258125e1_f64 * t49043 + t41959 + t41962 - 0.10064166666666666667e1_f64 * t47781 - 0.543465e1_f64 * t47785 + 0.31310740740740740741e0_f64 * t47787 - 0.3883875e1_f64 * t49049 + 0.247573125e0_f64 * t49052 + 0.247573125e0_f64 * t49054 + 0.82524375e-1_f64 * t49056 - 0.3883875e1_f64 * t49058 + 0.258925e1_f64 * t49060;
    t49219
}
