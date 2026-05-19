//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 337/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk337<F: Float>(t1006: F, t199: F, t591: F, t950: F, t590: F, t587: F, t1000: F, t606: F, t1002: F, t25: F, t599: F, t604: F) -> (F, F, F, F, F, F) {
    let t1008 = F::new(2.0) / F::new(15.0) * t1006 * t199;
    let t1009 = t591 * t950;
    let t1010 = t590 * t1009;
    let t1012 = F::new(4.0) / F::new(45.0) * t587 * t1010;
    let t1014 = t606 * t1000;
    let t1017 = -t599 - F::cast_from(0.35991666666666666667e-1_f64) * t1002 - t604 - F::cast_from(0.66666666666666666667e-2_f64) * t25 * t1014;
    (t1008, t1009, t1010, t1012, t1014, t1017)
}
