//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1221/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1221<F: Float>(t3966: F, t51966: F, t326: F, t378: F, t6594: F, t745: F, t837: F, t2306: F, t938: F, t1477: F, t274: F, t833: F, t850: F, t851: F) -> (F, F, F, F, F, F) {
    let t51967 = t51966 * t3966;
    let t51977 = t326 * t6594 * t378;
    let t51989 = t745 * t837;
    let t52000 = t2306 * t938;
    let t52033 = t274 * t1477;
    let t52036 = t850 * t851 * t52033 * t833;
    (t51967, t51977, t51989, t52000, t52033, t52036)
}
