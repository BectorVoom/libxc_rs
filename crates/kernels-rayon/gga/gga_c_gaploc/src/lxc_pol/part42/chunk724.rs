//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 724/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk724(t14361: f64, t14395: f64, t14402: f64, t14406: f64, t13338: f64, t13345: f64, t13352: f64, t13573: f64, t13587: f64, t14292: f64, t14297: f64, t14349: f64, t14350: f64, t1960: f64, t2969: f64, t3749: f64, t748: f64) -> (f64, f64) {
    let t14408 = t14361 + t14395 + t14402 + t14406;
    let t14412 = 4.0_f64 * t14350 * t1960 - t14408 * t748 - 2.0_f64 * t2969 * t3749 - t13338 + t13345 - t13352 + t13573 + t13587 + t14292 - t14297 + t14349;
    (t14408, t14412)
}
