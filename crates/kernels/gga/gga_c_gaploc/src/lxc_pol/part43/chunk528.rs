//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 528/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk528<F: Float>(t3177: F, t6985: F, t2487: F, t589: F, t587: F, t2365: F, t6510: F, t4391: F, t544: F, t6851: F, t2326: F, t900: F) -> (F, F, F, F, F, F, F) {
    let t9552 = t6985 * t3177;
    let t9553 = t2487 * t9552;
    let t9554 = F::new(0.51123901271894332901e0) * t9553;
    let t9555 = t589 * t3177;
    let t9556 = t587 * t9555;
    let t9557 = F::new(0.51123901271894332901e0) * t9556;
    let t9558 = t2365 * t6510;
    let t9560 = F::new(0.59584149919750711116e-1) * t4391 * t9558;
    let t9561 = t544 * t6851;
    let t9562 = t900 * t2326;
    (t9553, t9554, t9556, t9557, t9560, t9561, t9562)
}
