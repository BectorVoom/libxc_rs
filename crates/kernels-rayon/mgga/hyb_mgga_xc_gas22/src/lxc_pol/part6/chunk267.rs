//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 267/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk267(t260: f64, t271: f64, t785: f64, t813: f64, t816: f64, t821: f64, t830: f64, t836: f64, t840: f64, t849: f64, t856: f64, t858: f64) -> f64 {
    let t861 = -t785 + t813 + t260 * (-0.310907e-1_f64 * t816 * t271 + 1.0_f64 * t821 * t830 + t785 - t813 - 0.19751673498613801407e-1_f64 * t836 + 0.5848223622634646207e0_f64 * t840 * t849) + 0.19751673498613801407e-1_f64 * t260 * t836 - 0.5848223622634646207e0_f64 * t856 * t858;
    t861
}
