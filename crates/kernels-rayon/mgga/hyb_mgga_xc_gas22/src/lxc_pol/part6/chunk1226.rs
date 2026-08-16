//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1226/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1226(t2004: f64, t3157: f64, t677: f64, t8532: f64, t136: f64, t1815: f64, t3287: f64, t1240: f64, t6466: f64, t8545: f64, t3273: f64, t23127: f64, t3124: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23994 = t2004 * t3157;
    let t23996 = t677 * t8532;
    let t23999 = t136 * t1815 * t3287;
    let t24001 = t1240 * t6466;
    let t24003 = t677 * t8545;
    let t24006 = t136 * t1815 * t3273;
    let t24011 = t23127 * t3124;
    (t23994, t23996, t23999, t24001, t24003, t24006, t24011)
}
