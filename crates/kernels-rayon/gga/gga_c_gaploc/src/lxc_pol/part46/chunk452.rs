//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 452/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk452(t158: f64, t2293: f64, t2353: f64, t501: f64, t1381: f64, t892: f64, t60: f64, t78: f64) -> (f64, f64, f64, f64) {
    let t6540 = t158 * t2293;
    let t6553 = t2353 * t501;
    let t6556 = t892 * t1381;
    let t6574 = t60 * t78;
    (t6540, t6553, t6556, t6574)
}
