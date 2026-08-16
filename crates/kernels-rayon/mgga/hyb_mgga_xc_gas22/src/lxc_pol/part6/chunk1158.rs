//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1158/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1158(t1954: f64, t81: f64, t6091: f64, t622: f64, t1815: f64, t19: f64, t1996: f64, t2987: f64, t555: f64, t560: f64, t1783: f64, t6160: f64) -> (f64, f64, f64, f64, f64) {
    let t19975 = t81 * t1954;
    let t19990 = 1.0_f64 / t6091 / t622;
    let t20006 = t19 * t1815 * t1996;
    let t20022 = t555 * t2987 * t560;
    let t20070 = t555 * t6160 * t1783;
    (t19975, t19990, t20006, t20022, t20070)
}
