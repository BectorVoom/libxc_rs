//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 995/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk995(t3566: f64, t9333: f64, t2365: f64, t35913: f64, t4391: f64, t36274: f64, t6963: f64, t13437: f64, t1562: f64, t4614: f64, t42412: f64, t42431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46778 = 0.25025342966295298669e1_f64 * t3566 * t9333;
    let t46784 = t4391 * t2365 * t35913;
    let t46785 = 0.59584149919750711116e-1_f64 * t46784;
    let t46787 = t6963 * t2365 * t36274;
    let t46788 = 0.29792074959875355558e-1_f64 * t46787;
    let t46792 = 0.92023022289409799224e1_f64 * t1562 * t4614 * t13437;
    let t46793 = 0.11916829983950142223e0_f64 * t42412;
    let t46799 = 0.19171462976960374838e0_f64 * t42431;
    (t46778, t46785, t46788, t46792, t46793, t46799)
}
