//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1059/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1059<F: Float>(t3277: F, t44090: F, t13009: F, t5782: F, t1457: F, t43240: F, t6060: F, t13158: F, t15766: F, t41430: F, t41435: F, t41445: F) -> (F, F, F, F, F, F, F) {
    let t44092 = F::new(0.16683561977530199113e1) * t3277 * t44090;
    let t44093 = t5782 * t13009;
    let t44097 = F::new(0.21450293971110256001e1) * t6060 * t1457 * t43240;
    let t44099 = F::new(0.21450293971110256001e1) * t15766 * t13158;
    let t44110 = F::new(0.19171462976960374838e1) * t41430;
    let t44111 = F::new(0.42603251059911944084e0) * t41435;
    let t44112 = F::new(0.29792074959875355558e-1) * t41445;
    (t44092, t44093, t44097, t44099, t44110, t44111, t44112)
}
