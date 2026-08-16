//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1207/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1207<F: Float>(t745: F, t837: F, t833: F, t850: F, t851: F, t1477: F, t274: F, t14169: F, t804: F, t14360: F, t945: F, t321: F) -> (F, F, F, F, F, F, F) {
    let t51989 = t745 * t837;
    let t51992 = t850 * t851 * t51989 * t833;
    let t52033 = t274 * t1477;
    let t52036 = t850 * t851 * t52033 * t833;
    let t52079 = t804 * t14169;
    let t52089 = t14360 * t945;
    let t52090 = t321 * t52089;
    (t51989, t51992, t52033, t52036, t52079, t52089, t52090)
}
