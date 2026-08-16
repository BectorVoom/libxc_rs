//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1151/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1151(t2006: f64, t552: f64, t1307: f64, t6637: f64, t6888: f64, t794: f64, t8479: f64, t6897: f64, t1351: f64, t550: f64, t6976: f64, t1992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31193 = t552 * t2006;
    let t31194 = t31193 * t1307;
    let t31195 = t6637 * t31194;
    let t31197 = 0.3289868133696452873e-1_f64 * t6888 * t31195;
    let t31198 = t794 * t8479;
    let t31200 = 0.82246703342411321825e-2_f64 * t6897 * t31198;
    let t31201 = t2006 * t1351;
    let t31202 = t31201 * t550;
    let t31203 = t6976 * t31202;
    let t31205 = 0.16449340668482264365e-1_f64 * t1992 * t31203;
    (t31193, t31194, t31195, t31197, t31198, t31200, t31202, t31203, t31205)
}
