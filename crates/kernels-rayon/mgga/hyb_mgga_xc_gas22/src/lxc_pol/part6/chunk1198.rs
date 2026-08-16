//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1198/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1198(t1110: f64, t21846: f64, t22181: f64, t7241: f64, t2649: f64, t7245: f64, t2643: f64, t7255: f64, t2742: f64, t2754: f64, t2751: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22185 = 0.12304822629859687989e5_f64 * t1110 * t22181 * t21846 * t7241;
    let t22186 = t7245 * t2649;
    let t22189 = t2643 * t7255;
    let t22191 = t2754 * t2742;
    let t22193 = t2751 * t2742;
    let t22195 = t460 * t460;
    (t22185, t22186, t22189, t22191, t22193, t22195)
}
