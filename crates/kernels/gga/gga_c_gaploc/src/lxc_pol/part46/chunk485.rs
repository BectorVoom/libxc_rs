//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 485/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk485<F: Float>(t1: F, t3085: F, t106: F, t192: F, t3152: F, t528: F, t3148: F, t1564: F, t3116: F, t475: F, t1445: F, t4529: F, t9198: F, t9172: F, t2462: F, t60: F) -> (F, F, F, F, F, F, F, F) {
    let t9391 = t3085 * t1;
    let t9392 = t9391 * t106;
    let t9393 = t9392 * t192;
    let t9396 = t528 * t3152;
    let t9399 = t528 * t3148;
    let t9402 = t1564 * t3116;
    let t9403 = t9402 * t475;
    let t9404 = t1445 * t9403;
    let t9407 = t4529 * t3085;
    let t9408 = t9407 * t475;
    let t9409 = t1445 * t9408;
    let t9412 = t9198 * t475;
    let t9413 = t1445 * t9412;
    let t9416 = t1445 * t9172;
    let t9419 = t60 * t2462;
    (t9393, t9396, t9399, t9404, t9409, t9413, t9416, t9419)
}
