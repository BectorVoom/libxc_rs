//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 836/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk836(t20368: f64, t44386: f64, t1358: f64, t23915: f64, t161: f64, t37573: f64, t2339: f64, t172: f64, t3338: f64, t550: f64, t13396: f64, t2299: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44387 = t20368 * t44386;
    let t44390 = 0.18970004423784099732e-1_f64 * t1358 * t23915 * t44387;
    let t44391 = t37573 * t161;
    let t44394 = 0.94850022118920498663e-2_f64 * t1358 * t44391 * t2339;
    let t44395 = t172 * t3338;
    let t44396 = t550 * t44395;
    let t44403 = 0.31616674039640166221e-2_f64 * t1358 * t2299 * t13396 * t488;
    (t44387, t44390, t44394, t44395, t44396, t44403)
}
