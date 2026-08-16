//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1086/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1086(t197: f64, t3689: f64, t161: f64, t1358: f64, t2339: f64, t13735: f64, t6305: f64, t2268: f64, t2440: f64, t3691: f64, t13751: f64, t419: f64) -> (f64, f64, f64, f64, f64) {
    let t47008 = t197 * t3689;
    let t47009 = t47008 * t161;
    let t47011 = t1358 * t47009 * t2339;
    let t47013 = t6305 * t13735;
    let t47016 = t2268 * t2440 * t3691;
    let t47019 = 0.28455006635676149599e-1_f64 * t419 * t13751;
    (t47008, t47011, t47013, t47016, t47019)
}
