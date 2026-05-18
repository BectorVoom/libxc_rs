//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 224/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk224<F: Float>(t10: F, t671: F, t670: F, t395: F, t401: F, t7: F, t226: F, t230: F, t242: F, t528: F, t3: F, t551: F) -> (F, F, F, F, F, F, F, F, F) {
    let t672 = t10 * t671;
    let t674 = F::new(0.10821041362364843377e0) * t670 * t672;
    let t677 = F::new(0.4125e0) * t395 - t401 / F::new(6.0);
    let t678 = t677 * M_PI;
    let t679 = t678 * t7;
    let t681 = F::new(4.0) / F::new(3.0) * t226 * t679;
    let t683 = F::new(4.0) / F::new(3.0) * t226 * t230;
    let t692 = F::new(0.83762820535504401876e-1) * t528 * t242;
    let t696 = t551 * t3;
    (t672, t674, t677, t678, t679, t681, t683, t692, t696)
}
