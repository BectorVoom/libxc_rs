//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 443/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk443(t2124: f64, t551: f64, t570: f64, t7778: f64, t305: f64, t2064: f64, t793: f64, t2295: f64, t6444: f64, t1587: f64, t645: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8994 = t2124 * t551;
    let t8997 = t7778 * t570;
    let t8998 = t305 * t8997;
    let t9000 = t2064 * t551;
    let t9001 = t793 * t9000;
    let t9003 = t6444 * t2295;
    let t9005 = t645 * t1587;
    let t9006 = t793 * t9005;
    let t9008 = t2064 * t558;
    (t8994, t8998, t9001, t9003, t9006, t9008)
}
