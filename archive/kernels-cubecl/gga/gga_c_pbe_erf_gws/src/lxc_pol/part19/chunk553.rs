//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 553/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk553<F: Float>(t140: F, t1503: F, t142: F, t967: F, t1516: F, t485: F, t971: F, t395: F, t102: F, t481: F, t974: F, t2478: F, t2481: F, t2486: F, t2489: F) -> (F, F, F, F, F, F, F, F) {
    let t2857 = t1503 * t140;
    let t2858 = t142 * t967;
    let t2862 = F::cast_from(0.48717083333333333333e0_f64) * t1516;
    let t2863 = t485 * t971;
    let t2864 = t2863 * t395;
    let t2865 = F::cast_from(0.48717083333333333333e0_f64) * t2864;
    let t2868 = F::cast_from(0.584605e1_f64) * t102 * t974 * t481;
    let t2873 = -t2478 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2481 - t2486 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t2489;
    (t2857, t2858, t2862, t2863, t2864, t2865, t2868, t2873)
}
