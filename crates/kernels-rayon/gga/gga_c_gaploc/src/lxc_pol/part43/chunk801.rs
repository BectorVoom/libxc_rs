//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 801/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk801(t2487: f64, t2488: f64, t40190: f64, t2365: f64, t29985: f64, t4391: f64, t1429: f64, t30140: f64, t29854: f64, t29970: f64, t6963: f64, t12526: f64, t587: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40546 = t2487 * t2488 * t40190;
    let t40549 = t4391 * t2365 * t29985;
    let t40555 = t1429 * t2365 * t30140;
    let t40558 = t4391 * t2365 * t29854;
    let t40561 = t6963 * t2365 * t29970;
    let t40564 = t587 * t589 * t12526;
    (t40546, t40549, t40555, t40558, t40561, t40564)
}
