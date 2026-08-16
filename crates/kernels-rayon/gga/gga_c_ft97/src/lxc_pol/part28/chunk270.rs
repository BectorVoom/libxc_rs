//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 270/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk270(t2993: f64, t447: f64, t446: f64, t14: f64, t1576: f64, t17: f64, t355: f64, t18: f64, t359: f64, t89: f64, t375: f64, t943: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2994 = t447 * t2993;
    let t2995 = t446 * t2994;
    let t2998 = 1.0_f64 / t14 / t1576;
    let t2999 = t2998 * t17;
    let t3000 = t2999 * t355;
    let t3001 = t359 * t18;
    let t3003 = t89 * t3000 * t3001;
    let t3006 = t89 * t375 * t943;
    (t2995, t2998, t2999, t3000, t3003, t3006)
}
