//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1295/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1295(t10477: f64, t7884: f64, t10467: f64, t677: f64, t10445: f64, t136: f64, t550: f64, t1815: f64, t4088: f64, t1240: f64, t8223: f64, t8453: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28106 = t7884 * t10477;
    let t28108 = t677 * t10467;
    let t28111 = t136 * t550 * t10445;
    let t28115 = t136 * t1815 * t4088;
    let t28119 = t1240 * t8223;
    let t28121 = t1240 * t8453;
    (t28106, t28108, t28111, t28115, t28119, t28121)
}
