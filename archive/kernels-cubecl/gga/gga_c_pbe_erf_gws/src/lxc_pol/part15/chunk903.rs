//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 903/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk903<F: Float>(t639: F, t7874: F, t219: F, t5480: F, t2679: F, t1027: F, t1724: F, t1815: F, t1809: F, t7264: F, t2580: F, t5125: F) -> (F, F, F, F, F) {
    let t7876 = F::cast_from(32.0_f64) / F::cast_from(135.0_f64) * t639 * t7874;
    let t7877 = t5480 * t219;
    let t7878 = t7877 * t2679;
    let t7880 = F::cast_from(16.0_f64) / F::cast_from(81.0_f64) * t639 * t7878;
    let t7881 = t1027 * t1724;
    let t7882 = t1815 * t7881;
    let t7884 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t639 * t7882;
    let t7885 = t1809 * t7264;
    let t7887 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t639 * t7885;
    let t7888 = t5125 * t2580;
    (t7876, t7880, t7884, t7887, t7888)
}
