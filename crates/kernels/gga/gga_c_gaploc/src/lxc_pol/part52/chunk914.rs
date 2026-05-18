//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 914/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk914<F: Float>(t11218: F, t123: F, t883: F, t2487: F, t2488: F, t11254: F, t2464: F, t2465: F, t10532: F, t10533: F, t46362: F, t37675: F, t901: F) -> (F, F, F, F, F) {
    let t46401 = t11218 * t123 * t883;
    let t46403 = t2487 * t2488 * t46401;
    let t46404 = F::new(0.19171462976960374838e0) * t46403;
    let t46407 = t2487 * t2464 * t2465 * t11254;
    let t46408 = F::new(0.42603251059911944084e-1) * t46407;
    let t46420 = F::new(0.38649669361552115674e3) * t10532 * t10533 * t46362;
    let t46421 = t37675 * t901;
    (t46401, t46404, t46408, t46420, t46421)
}
