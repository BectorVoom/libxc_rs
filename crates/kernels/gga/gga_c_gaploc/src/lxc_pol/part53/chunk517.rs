//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 517/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk517<F: Float>(t475: F, t9407: F, t1445: F, t9198: F, t9172: F, t2462: F, t60: F) -> (F, F, F, F) {
    let t9408 = t9407 * t475;
    let t9409 = t1445 * t9408;
    let t9412 = t9198 * t475;
    let t9413 = t1445 * t9412;
    let t9416 = t1445 * t9172;
    let t9419 = t60 * t2462;
    (t9409, t9413, t9416, t9419)
}
