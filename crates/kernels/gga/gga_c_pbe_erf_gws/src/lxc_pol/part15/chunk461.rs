//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 461/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk461<F: Float>(t573: F, t610: F, t1827: F, t587: F, t108: F, t1403: F, t1407: F, t1413: F, t1416: F, t726: F, t728: F, t92: F, t93: F) -> (F, F, F, F) {
    let t1828 = t573 * t610;
    let t1829 = t1827 * t1828;
    let t1831 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t587 * t1829;
    let t1841 = (F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t92 * t1403 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t726 * t1407 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t93 * t1413 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t728 * t1416) * t108;
    (t1828, t1829, t1831, t1841)
}
