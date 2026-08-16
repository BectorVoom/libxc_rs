//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1057/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1057<F: Float>(t10599: F, t1547: F, t2799: F, t13615: F, t894: F, t1553: F, t2403: F, t4392: F, t699: F, t13611: F, t908: F, t136: F) -> (F, F, F, F, F, F) {
    let t13637 = t10599 * t1547;
    let t13638 = t13637 * t2799;
    let t13640 = t894 * t13615;
    let t13642 = t2403 * t1553;
    let t13644 = t699 * t4392;
    let t13645 = F::cast_from(0.10954222222222222222e0_f64) * t13644;
    let t13646 = t908 * t13611;
    let t13647 = t136 * t13646;
    (t13638, t13640, t13642, t13644, t13645, t13647)
}
