//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1071/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1071(t31198: f64, t6897: f64, t1351: f64, t2006: f64, t550: f64, t6976: f64, t1992: f64, t1998: f64, t6955: f64, t214: f64, t1985: f64, t1338: f64, t8470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31200 = 0.82246703342411321825e-2_f64 * t6897 * t31198;
    let t31201 = t2006 * t1351;
    let t31202 = t31201 * t550;
    let t31203 = t6976 * t31202;
    let t31205 = 0.16449340668482264365e-1_f64 * t1992 * t31203;
    let t31206 = t1998 * t6955;
    let t31207 = t214 * t31206;
    let t31209 = 0.16449340668482264365e-1_f64 * t1985 * t31207;
    let t31211 = t1338 * t8470;
    (t31200, t31202, t31203, t31205, t31206, t31207, t31209, t31211)
}
