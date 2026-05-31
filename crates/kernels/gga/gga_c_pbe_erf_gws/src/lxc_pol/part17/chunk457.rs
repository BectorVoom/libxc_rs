//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 457/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk457<F: Float>(t617: F, t626: F, t422: F, t1809: F, t1620: F, t642: F, t649: F) -> (F, F, F, F) {
    let t1810 = t617 * t626;
    let t1811 = t1810 * t422;
    let t1812 = t1809 * t1811;
    let t1814 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1620 * t1812;
    let t1815 = t642 * t649;
    (t1811, t1812, t1814, t1815)
}
