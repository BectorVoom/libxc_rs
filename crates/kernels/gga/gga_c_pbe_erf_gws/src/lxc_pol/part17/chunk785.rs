//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 785/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk785<F: Float>(t496: F, t5818: F, t505: F, t96: F, t1235: F, t125: F, t128: F, t2: F, t39: F, t1570: F, t513: F, t1576: F, t510: F) -> (F, F, F, F, F) {
    let t5819 = t496 * t5818;
    let t5825 = F::new(1.0) / t505 / t96;
    let t5832 = t125 * t1235;
    let t5833 = t128 * t2;
    let t5836 = F::cast_from(0.32645333333333333334e0_f64) * t5832 * t5833 * t39;
    let t5844 = t1570 * t513;
    let t5847 = t510 * t1576;
    (t5819, t5825, t5836, t5844, t5847)
}
