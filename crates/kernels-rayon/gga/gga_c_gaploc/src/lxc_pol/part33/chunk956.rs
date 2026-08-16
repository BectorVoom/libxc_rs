//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 956/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk956(t10627: f64, t7290: f64, t7289: f64, t1841: f64, t2576: f64, t8878: f64, t2536: f64, t3487: f64, t734: f64, t3009: f64, t7291: f64, t7226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10628 = t7290 * t10627;
    let t10629 = t7289 * t10628;
    let t10631 = 0.17090058289204942852e-2_f64 * t1841 * t10629;
    let t10632 = t8878 * t2576;
    let t10634 = 0.25635087433807414279e-2_f64 * t1841 * t10632;
    let t10635 = t2536 * t3487;
    let t10636 = t10635 * t734;
    let t10638 = 0.85450291446024714263e-3_f64 * t1841 * t10636;
    let t10639 = t3009 * t7291;
    let t10640 = t7226 * t10639;
    (t10628, t10629, t10631, t10632, t10634, t10636, t10638, t10639, t10640)
}
