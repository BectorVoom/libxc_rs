//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 926/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk926<F: Float>(t46641: F, t46103: F, t6963: F, t6964: F, t13465: F, t1407: F, t10430: F, t10608: F, t9272: F, t11400: F, t1424: F, t2299: F, t544: F) -> (F, F, F, F, F) {
    let t46642 = F::cast_from(0.42603251059911944084e-1_f64) * t46641;
    let t46645 = F::cast_from(0.71500979903700853338e0_f64) * t6963 * t6964 * t46103;
    let t46646 = t1407 * t13465;
    let t46653 = t9272 * t10608 * t10430;
    let t46654 = F::cast_from(0.11502877786176224903e1_f64) * t46653;
    let t46658 = F::cast_from(0.39722766613167140743e-1_f64) * t544 * t2299 * t11400 * t1424;
    (t46642, t46645, t46646, t46654, t46658)
}
