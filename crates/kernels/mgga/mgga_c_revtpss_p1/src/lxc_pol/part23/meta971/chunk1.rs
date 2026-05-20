//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3280/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3280<F: Float>(t22815: F, t3989: F, t221: F, t22813: F, t3978: F, t46716: F, t1883: F, t22020: F, t2661: F, t3992: F, t22877: F, t46691: F) -> (F, F, F, F) {
    let t86222 = t3989 * t22815;
    let t86226 = t3978 * t46716 * t221 * t22813;
    let t86234 = t2661 * t3992 * t22020 * t1883;
    let t86236 = t46691 * t22877;
    (t86222, t86226, t86234, t86236)
}
