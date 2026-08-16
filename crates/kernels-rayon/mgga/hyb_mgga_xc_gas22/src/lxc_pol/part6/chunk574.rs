//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 574/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk574(t1047: f64, t2713: f64, t2712: f64, t2657: f64, t2660: f64, t2663: f64, t2667: f64, t2669: f64, t2672: f64) -> (f64, f64, f64) {
    let t2714 = t2713 * t1047;
    let t2716 = 2.0_f64 * t2712 * t2714;
    let t2723 = -0.42198333333333333333e0_f64 * t2657 + 0.84396666666666666666e0_f64 * t2660 + 0.39862222222222222223e0_f64 * t2663 + 0.68258333333333333333e-1_f64 * t2667 + 0.13651666666666666667e0_f64 * t2669 + 0.13692777777777777778e0_f64 * t2672;
    (t2714, t2716, t2723)
}
