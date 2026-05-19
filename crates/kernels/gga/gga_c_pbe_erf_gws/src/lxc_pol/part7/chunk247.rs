//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 247/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk247<F: Float>(t10: F, t671: F, t670: F, t395: F, t401: F, t7: F, t226: F, t230: F, t231: F, t566: F, t581: F, t585: F, t595: F, t614: F, t621: F, t635: F, t638: F, t647: F, t665: F, t666: F) -> (F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t672 = t10 * t671;
    let t674 = F::cast_from(0.10821041362364843377e0_f64) * t670 * t672;
    let t677 = F::new(0.4125e0) * t395 - t401 / F::new(6.0);
    let t678 = t677 * pi;
    let t679 = t678 * t7;
    let t681 = F::new(4.0) / F::new(3.0) * t226 * t679;
    let t683 = F::new(4.0) / F::new(3.0) * t226 * t230;
    let t684 = t566 + t581 + t585 + t595 - t614 + t621 + t635 + t638 + t647 - t665 + F::new(4.0) / F::new(3.0) * t666 * t231 + t674 + t681 + t683;
    (t672, t677, t678, t679, t684)
}
