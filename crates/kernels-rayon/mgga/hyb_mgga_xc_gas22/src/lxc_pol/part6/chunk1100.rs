//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1100/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1100(t10557: f64, t10559: f64, t10561: f64, t10563: f64, t10565: f64, t10619: f64, t10621: f64, t10635: f64, t10637: f64, t10640: f64, t10643: f64, t3399: f64, t3419: f64, t4181: f64, t6716: f64) -> f64 {
    let t10776 = 0.11696447245269292414e1_f64 * t3399 * t3419 - 0.11696447245269292414e1_f64 * t6716 * t4181 - t10557 - t10559 - t10561 + t10563 - t10565 - t10619 - t10621 + t10635 - t10637 - t10640 + t10643;
    t10776
}
