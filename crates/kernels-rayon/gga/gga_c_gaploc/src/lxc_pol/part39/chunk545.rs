//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 545/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk545(t475: f64, t9407: f64, t1445: f64, t9198: f64, t9172: f64, t2462: f64, t60: f64) -> (f64, f64, f64, f64) {
    let t9408 = t9407 * t475;
    let t9409 = t1445 * t9408;
    let t9412 = t9198 * t475;
    let t9413 = t1445 * t9412;
    let t9416 = t1445 * t9172;
    let t9419 = t60 * t2462;
    (t9409, t9413, t9416, t9419)
}
