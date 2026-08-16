//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1413/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1413(t1100: f64, t11217: f64, t462: f64, t2813: f64, t4475: f64, t22173: f64, t22175: f64, t22179: f64, t22185: f64, t22186: f64, t22189: f64, t22191: f64, t22193: f64, t22199: f64, t22204: f64, t22208: f64, t22210: f64, t22212: f64, t22215: f64, t22400: f64, t30503: f64, t495: f64) -> f64 {
    let t30538 = t462 * t11217 * t1100;
    let t30547 = t462 * t4475 * t2813;
    let t30548 = -8.0_f64 * t22173 + 64.0_f64 * t22175 - t22179 + t22185 + 0.11696447245269292414e1_f64 * t22186 + 0.20779030926817756511e3_f64 * t22189 - 24.0_f64 * t22191 + 120.0_f64 * t22193 + 2.0_f64 * t30538 + t22199 - 480.0_f64 * t22204 + 20.0_f64 * t22208 + 12.0_f64 * t22210 + 32.0_f64 * t22212 + t462 * t30503 * t495 + t30547 - t22215 + t22400;
    t30548
}
