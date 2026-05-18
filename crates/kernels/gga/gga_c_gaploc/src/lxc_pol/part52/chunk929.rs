//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 929/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk929<F: Float>(t42378: F, t11433: F, t1415: F, t7030: F, t11426: F, t9562: F, t11318: F, t1445: F, t2293: F, t574: F, t13475: F, t1580: F) -> (F, F, F, F, F) {
    let t46705 = F::new(0.25561950635947166451e0) * t42378;
    let t46707 = t1415 * t11433 * t7030;
    let t46708 = F::new(0.14896037479937677779e-1) * t46707;
    let t46709 = t11426 * t9562;
    let t46715 = F::new(0.92023022289409799224e1) * t574 * t1445 * t11318 * t2293;
    let t46717 = F::new(0.43710935587469654631e2) * t1580 * t13475;
    (t46705, t46708, t46709, t46715, t46717)
}
