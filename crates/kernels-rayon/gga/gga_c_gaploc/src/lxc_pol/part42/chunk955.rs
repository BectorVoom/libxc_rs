//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 955/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk955(t1063: f64, t11977: f64, t3701: f64, t44328: f64, t44334: f64, t44336: f64, t44350: f64, t44355: f64, t44358: f64, t44363: f64, t44367: f64, t44371: f64, t44375: f64, t44377: f64, t44390: f64, t44394: f64, t44403: f64, t44409: f64, t44410: f64, t7974: f64, t8207: f64) -> f64 {
    let t49859 = t44328 + t44334 + t44336 - t44350 - t44355 - t44358 + t44363 - t44367 + t44371 - t44375 + t44377 + 0.1707300398140568976e0_f64 * t1063 * t11977 * t8207 - 0.56910013271352299198e-1_f64 * t1063 * t3701 * t7974 + t44390 + t44394 - t44403 + t44409 - t44410;
    t49859
}
