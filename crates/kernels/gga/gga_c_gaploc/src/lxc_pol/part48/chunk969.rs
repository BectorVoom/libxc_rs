//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 969/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk969<F: Float>(t41914: F, t41947: F, t41949: F, t41951: F, t10430: F, t2890: F, t9267: F, t2478: F, t3545: F, t6576: F, t2482: F, t3536: F, t9263: F) -> (F, F, F, F, F, F, F) {
    let t46368 = F::new(0.59584149919750711116e-1) * t41914;
    let t46370 = F::new(0.17875244975925213335e0) * t41947;
    let t46371 = F::new(0.17875244975925213335e0) * t41949;
    let t46372 = F::new(0.17875244975925213335e0) * t41951;
    let t46378 = t9267 * t2890 * t10430;
    let t46379 = F::new(0.19171462976960374838e1) * t46378;
    let t46381 = t6576 * t3545 * t2478;
    let t46382 = F::new(0.19171462976960374838e0) * t46381;
    let t46384 = t9263 * t3536 * t2482;
    (t46368, t46370, t46371, t46372, t46379, t46382, t46384)
}
