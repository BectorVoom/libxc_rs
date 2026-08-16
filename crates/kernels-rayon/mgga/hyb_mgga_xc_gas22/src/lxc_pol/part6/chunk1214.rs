//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1214/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1214(t1815: f64, t19: f64, t3114: f64, t3118: f64, t7884: f64, t8169: f64, t1819: f64, t555: f64, t7909: f64, t7898: f64, t8185: f64, t20685: f64, t24: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23572 = t19 * t1815 * t3114;
    let t23575 = t19 * t1815 * t3118;
    let t23577 = t7884 * t8169;
    let t23588 = t555 * t1819 * t7909;
    let t23591 = t555 * t8185 * t7898;
    let t23622 = t24 * t20685;
    (t23572, t23575, t23577, t23588, t23591, t23622)
}
