//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1154/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1154(t12283: f64, t12404: f64, t12413: f64, t12267: f64, t3802: f64, t3734: f64, t3792: f64, t12279: f64, t16398: f64, t12409: f64, t3719: f64, t12167: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39971 = t12283 * t12404;
    let t39973 = t12283 * t12413;
    let t39975 = t12267 * t3802;
    let t39978 = t3792 * t3734;
    let t39983 = t16398 * t12279;
    let t39989 = t12283 * t12409;
    let t39993 = t3792 * t3719;
    let t40000 = t3792 * t12167;
    (t39971, t39973, t39975, t39978, t39983, t39989, t39993, t40000)
}
