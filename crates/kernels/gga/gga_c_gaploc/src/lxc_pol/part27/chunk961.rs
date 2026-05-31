//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 961/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk961<F: Float>(t10657: F, t471: F, t3427: F, t64: F, t9664: F, t9666: F, t9674: F, t9676: F, t10627: F, t688: F, t779: F, t2508: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10658 = t10657 * t471;
    let t10660 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3427 * t64;
    let t10663 = F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t9664;
    let t10664 = F::cast_from(21.0_f64) / F::cast_from(8192.0_f64) * t9666;
    let t10665 = F::cast_from(7.0_f64) / F::cast_from(8192.0_f64) * t9674;
    let t10666 = F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t9676;
    let t10682 = t10627 * t688;
    let t10683 = t779 * t10682;
    let t10685 = F::cast_from(0.76905262301422242837e-2_f64) * t2508 * t10683;
    (t10658, t10660, t10663, t10664, t10665, t10666, t10682, t10683, t10685)
}
