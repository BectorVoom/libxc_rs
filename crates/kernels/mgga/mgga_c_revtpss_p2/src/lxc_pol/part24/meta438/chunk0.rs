//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1393/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1393<F: Float>(t1337: F, t40101: F, t1340: F, t40097: F, t39816: F, t1333: F, t9855: F, t19: F, t2237: F, t521: F, t9342: F, t14: F, t27: F) -> (F, F, F, F, F, F, F) {
    let t46992 = F::cast_from(0.18989649058080861537e-2_f64) * t1337 * t40101;
    let t46996 = F::cast_from(0.46785788981077169656e1_f64) * t1340 * t40097;
    let t46998 = F::cast_from(0.69263436422725855036e2_f64) * t1340 * t39816;
    let t46999 = t9855 * t1333;
    let t47000 = F::cast_from(576.0_f64) * t46999;
    let t47003 = F::cast_from(840.0_f64) * t19 * t2237 * t521;
    let t47013 = t9342 * t1333;
    let t47014 = F::cast_from(96.0_f64) * t47013;
    let t47016 = t14 * t27 * t521;
    (t46992, t46996, t46998, t47000, t47003, t47014, t47016)
}
