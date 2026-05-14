//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 804/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk804<F: Float>(t11318: F, t1445: F, t2293: F, t574: F, t13475: F, t1580: F, t10348: F, t11362: F, t11429: F, t1415: F, t7030: F, t13471: F, t7014: F, t2898: F, t44310: F, t900: F) -> (F, F, F, F, F, F) {
    let t46715 = 0.92023022289409799224e1 * t574 * t1445 * t11318 * t2293;
    let t46717 = 0.43710935587469654631e2 * t1580 * t13475;
    let t46724 = 0.7150097990370085334e0 * t11362 * t10348;
    let t46729 = t1415 * t11429 * t7030;
    let t46730 = 0.14896037479937677779e-1 * t46729;
    let t46731 = t7014 * t13471;
    let t46732 = 0.19171462976960374838e0 * t46731;
    let t46734 = t2898 * t900 * t44310;
    (t46715, t46717, t46724, t46730, t46732, t46734)
}
