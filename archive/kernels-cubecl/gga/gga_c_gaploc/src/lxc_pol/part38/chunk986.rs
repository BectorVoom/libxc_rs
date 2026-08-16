//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 986/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk986<F: Float>(t46637: F, t11167: F, t2464: F, t2465: F, t587: F, t46103: F, t6963: F, t6964: F, t13465: F, t1407: F, t10430: F, t10608: F, t9272: F) -> (F, F, F, F, F) {
    let t46638 = F::cast_from(0.19171462976960374838e0_f64) * t46637;
    let t46641 = t587 * t2464 * t2465 * t11167;
    let t46642 = F::cast_from(0.42603251059911944084e-1_f64) * t46641;
    let t46645 = F::cast_from(0.71500979903700853338e0_f64) * t6963 * t6964 * t46103;
    let t46646 = t1407 * t13465;
    let t46653 = t9272 * t10608 * t10430;
    (t46638, t46642, t46645, t46646, t46653)
}
