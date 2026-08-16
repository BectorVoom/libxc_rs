//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1041/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1041<F: Float>(t1105: F, t745: F, t1123: F, t2255: F, t2494: F, t274: F, t11499: F, t3257: F, t6196: F, t11478: F, t11633: F, t3139: F) -> (F, F, F, F) {
    let t11678 = t745 * t1105;
    let t11680 = t2255 * t1123 * t11678;
    let t11683 = t274 * t2494;
    let t11685 = t2255 * t1123 * t11683;
    let t11689 = t3257 * t11499 * t6196;
    let t11693 = t3139 * t11478 * t11633;
    (t11680, t11685, t11689, t11693)
}
