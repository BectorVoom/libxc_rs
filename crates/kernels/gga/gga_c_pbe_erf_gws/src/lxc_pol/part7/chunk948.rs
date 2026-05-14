//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 948/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk948<F: Float>(t1368: F, t1464: F, t285: F, t2036: F, t5887: F, t281: F, t4562: F, t545: F, t168: F, t18344: F, t286: F, t475: F, t5880: F, t1570: F, t510: F, t5651: F) -> (F, F, F, F, F, F) {
    let t19107 = 0.81358876250083374227e-2 * t1464 * t1368 * t285;
    let t19108 = t5887 * t2036;
    let t19117 = t281 * t4562 * t545 * t285;
    let t19121 = 0.91063310497738755577e0 * t168 * t18344 * t286;
    let t19124 = t475 * t5880;
    let t19129 = t5651 * t510 * t1570;
    (t19107, t19108, t19117, t19121, t19124, t19129)
}
