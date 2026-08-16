//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 843/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk843(t1: f64, t106: f64, t12792: f64, t192: f64, t12865: f64, t1580: f64, t31828: f64, t874: f64, t1445: f64, t597: f64, t10151: f64, t2293: f64) -> (f64, f64, f64, f64, f64) {
    let t41860 = t12792 * t1 * t106 * t192;
    let t41863 = t1580 * t12865;
    let t41865 = t31828 * t874;
    let t41867 = t597 * t1445 * t41865;
    let t41869 = t10151 * t2293;
    (t41860, t41863, t41865, t41867, t41869)
}
