//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2447/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2447<F: Float>(t4038: F, t9318: F, t1337: F, t40101: F, t9323: F, t1340: F, t40097: F, t39816: F, t19: F, t2237: F, t521: F, t1331: F, t9342: F) -> (F, F, F, F, F, F, F) {
    let t46989 = t4038 * t9318;
    let t46992 = F::cast_from(0.18989649058080861537e-2_f64) * t1337 * t40101;
    let t46993 = t4038 * t9323;
    let t46996 = F::cast_from(0.46785788981077169656e1_f64) * t1340 * t40097;
    let t46998 = F::cast_from(0.69263436422725855036e2_f64) * t1340 * t39816;
    let t47003 = F::cast_from(840.0_f64) * t19 * t2237 * t521;
    let t47005 = t9342 * t1331;
    (t46989, t46992, t46993, t46996, t46998, t47003, t47005)
}
