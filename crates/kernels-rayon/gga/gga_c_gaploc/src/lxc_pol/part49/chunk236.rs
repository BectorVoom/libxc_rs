//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 236/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk236(t191: f64, t203: f64, t107: f64, t19: f64, t594: f64, t1434: f64, t584: f64) -> (f64, f64, f64, f64) {
    let t1531 = t191 * t203;
    let t1532 = t107 * t1531;
    let t1535 = t594 * t19;
    let t1536 = t1535 * t1434;
    let t1537 = t584 * t1536;
    (t1531, t1532, t1535, t1537)
}
