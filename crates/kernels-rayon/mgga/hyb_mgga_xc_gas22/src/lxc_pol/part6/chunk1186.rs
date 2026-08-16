//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1186/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1186(t1057: f64, t7483: f64, t1052: f64, t7313: f64, t1048: f64, t2712: f64, t7435: f64, t2696: f64, t2813: f64, t462: f64, t2630: f64, t2649: f64, t2662: f64) -> (f64, f64, f64, f64, f64) {
    let t21975 = 16.0_f64 * t1057 * t7483;
    let t21978 = t1052 * t7313;
    let t21982 = 8.0_f64 * t2712 * t1048 * t7435;
    let t21984 = t462 * t2696 * t2813;
    let t21990 = 0.86748650402413918736e-1_f64 * t2630 * t2662 * t2649;
    (t21975, t21978, t21982, t21984, t21990)
}
