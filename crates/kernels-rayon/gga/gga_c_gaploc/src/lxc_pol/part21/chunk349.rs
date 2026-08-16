//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 349/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk349(t1529: f64, t188: f64, t191: f64, t203: f64, t107: f64, t19: f64, t594: f64) -> (f64, f64, f64, f64) {
    let t1530 = t188 * t1529;
    let t1531 = t191 * t203;
    let t1532 = t107 * t1531;
    let t1535 = t594 * t19;
    (t1530, t1531, t1532, t1535)
}
