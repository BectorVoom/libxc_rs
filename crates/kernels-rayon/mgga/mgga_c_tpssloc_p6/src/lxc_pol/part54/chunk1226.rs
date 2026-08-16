//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1226/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1226(t31366: f64, t7479: f64, t6552: f64, t7488: f64, t1880: f64, t225: f64, t258: f64, t7823: f64, t214: f64, t1911: f64, t7841: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33419 = t31366 * t7479;
    let t33420 = t6552 * t33419;
    let t33422 = t31366 * t7488;
    let t33423 = t1880 * t33422;
    let t33428 = t7823 * t225 * t258;
    let t33429 = t214 * t33428;
    let t33430 = t1880 * t33429;
    let t33432 = t7841 * t1911;
    let t33433 = t2718 * t33432;
    (t33419, t33420, t33422, t33423, t33428, t33429, t33430, t33433)
}
