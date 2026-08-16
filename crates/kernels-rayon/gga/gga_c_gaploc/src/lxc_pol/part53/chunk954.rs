//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 954/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk954(t2268: f64, t2440: f64, t3691: f64, t13751: f64, t419: f64, t13729: f64, t6305: f64, t38392: f64, t874: f64, t2343: f64, t1358: f64, t13777: f64, t2299: f64, t488: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47016 = t2268 * t2440 * t3691;
    let t47019 = 0.28455006635676149599e-1_f64 * t419 * t13751;
    let t47024 = t6305 * t13729;
    let t47026 = t38392 * t874;
    let t47028 = t2268 * t2343 * t47026;
    let t47032 = t1358 * t2299 * t13777 * t488;
    (t47016, t47019, t47024, t47026, t47028, t47032)
}
