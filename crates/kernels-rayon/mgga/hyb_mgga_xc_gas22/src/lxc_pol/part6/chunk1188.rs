//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1188/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1188(t2643: f64, t7269: f64, t2699: f64, t7520: f64, t7516: f64, t1110: f64, t2635: f64, t7410: f64, t7515: f64, t2636: f64, t2674: f64, t7237: f64, t7241: f64) -> (f64, f64, f64, f64, f64) {
    let t22010 = t2643 * t7269;
    let t22012 = t2699 * t7520;
    let t22014 = t2643 * t7516;
    let t22019 = 0.69263436422725855036e2_f64 * t1110 * t2635 * t7410 * t7515;
    let t22024 = 0.61524113149298439947e4_f64 * t1110 * t7237 * t2674 * t7241 * t2636;
    (t22010, t22012, t22014, t22019, t22024)
}
