//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1208/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1208<F: Float>(t188: F, t46965: F, t3377: F, t11977: F, t524: F, t13778: F, t2487: F, t6985: F, t1445: F, t46920: F, t597: F, t42395: F, t42398: F, t42401: F, t42405: F, t42407: F, t42413: F, t42416: F, t42421: F) -> F {
    let t48187 = t188 * t46965;
    let t48188 = t48187 * t3377;
    let t48190 = t524 * t11977;
    let t48191 = t48190 * t3377;
    let t48194 = t2487 * t6985 * t13778;
    let t48198 = F::new(0.11502877786176224903e2) * t597 * t1445 * t46920;
    let t48200 = -t42395 - t42398 - F::new(0.10725146985555128001e1) * t48188 - F::new(0.10725146985555128001e1) * t48191 - F::new(0.25561950635947166451e0) * t48194 + t48198 - t42401 - t42405 + t42407 - t42413 + F::new(0.42603251059911944084e-1) * t42416 - t42421;
    t48200
}
