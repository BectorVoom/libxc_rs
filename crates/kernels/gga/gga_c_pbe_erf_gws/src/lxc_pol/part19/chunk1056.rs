//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1056/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1056<F: Float>(t3792: F, t6183: F, t3116: F, t11844: F, t11846: F, t11849: F, t11852: F, t11854: F, t11857: F, t11862: F, t11863: F, t11864: F, t11867: F, t2253: F, t6456: F, t9539: F) -> (F, F) {
    let t11868 = t6183 * t3792;
    let t11869 = t3116 * t11868;
    let t11870 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t11869;
    let t11871 = t11844 - F::cast_from(119.0_f64) / F::cast_from(6912.0_f64) * t6456 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t11846 - t2253 * t11849 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t11852 - t2253 * t11854 / F::cast_from(768.0_f64) + t9539 - F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t11857 + t11862 - t11863 - F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t11864 - t11867 + t11870;
    (t11870, t11871)
}
