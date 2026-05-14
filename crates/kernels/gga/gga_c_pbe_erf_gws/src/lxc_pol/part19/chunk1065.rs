//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1065/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1065<F: Float>(t858: F, t892: F, t1114: F, t20112: F, t1105: F, t6854: F, t12041: F, t19894: F, t3028: F, t376: F, t4383: F, t9847: F, t3916: F, t4384: F, t19898: F, t3912: F) -> (F, F, F, F, F, F, F, F) {
    let t29751 = t858 * t892;
    let t29775 = t1114 * t20112;
    let t30104 = t6854 * t1105;
    let t34773 = t12041 * t19894;
    let t34838 = t376 * t3028;
    let t34850 = t1114 * t9847 * t4383;
    let t34922 = t3916 * t4384;
    let t35000 = t3912 * t19898;
    (t29751, t29775, t30104, t34773, t34838, t34850, t34922, t35000)
}
