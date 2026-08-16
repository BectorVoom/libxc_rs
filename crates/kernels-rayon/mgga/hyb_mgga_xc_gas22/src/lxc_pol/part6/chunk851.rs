//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 851/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk851(t2243: f64, t820: f64, t2272: f64, t816: f64, t2271: f64, t267: f64, t262: f64) -> (f64, f64, f64, f64) {
    let t6673 = t2243 * t820;
    let t6678 = t816 * t2272;
    let t6682 = 1.0_f64 / t2271 / t267;
    let t6683 = t262 * t6682;
    (t6673, t6678, t6682, t6683)
}
