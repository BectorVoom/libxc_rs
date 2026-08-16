//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1007/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1007(t12505: f64, t12526: f64, t12547: f64, t12568: f64, t11043: f64, t11046: f64, t1125: f64, t12464: f64, t12466: f64, t12476: f64, t12479: f64, t12483: f64, t2464: f64, t2469: f64, t338: f64, t3565: f64, t3568: f64, t3622: f64, t3883: f64, t3897: f64, t7056: f64, t7063: f64, t884: f64, t972: f64) -> (f64, f64) {
    let t12570 = t12505 + t12526 + t12547 + t12568;
    let t12572 = -2.0_f64 * t11043 * t1125 + 4.0_f64 * t11046 * t3568 + t12464 * t338 - t12466 * t972 - 6.0_f64 * t12476 * t7063 + 4.0_f64 * t12479 * t2469 + 2.0_f64 * t12483 * t2469 - t12570 * t884 - t2464 * t3897 - 2.0_f64 * t3565 * t3622 + 2.0_f64 * t3883 * t7056;
    (t12570, t12572)
}
