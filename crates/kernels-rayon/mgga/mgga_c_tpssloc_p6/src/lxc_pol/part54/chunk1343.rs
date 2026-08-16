//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1343/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1343(t120490: f64, t552: f64, t7722: f64, t1307: f64, t6637: f64, t6888: f64, t1992: f64, t26404: f64, t6976: f64, t22897: f64, t26453: f64, t114097: f64) -> (f64, f64, f64, f64, f64) {
    let t120491 = 0.16449340668482264365e-1_f64 * t120490;
    let t120492 = t552 * t7722;
    let t120496 = 0.3289868133696452873e-1_f64 * t6888 * t6637 * t120492 * t1307;
    let t120502 = 0.16449340668482264365e-1_f64 * t1992 * t6976 * t26404;
    let t120505 = 0.3289868133696452873e-1_f64 * t1992 * t22897 * t26453;
    let t120506 = 0.82246703342411321825e-2_f64 * t114097;
    (t120491, t120496, t120502, t120505, t120506)
}
