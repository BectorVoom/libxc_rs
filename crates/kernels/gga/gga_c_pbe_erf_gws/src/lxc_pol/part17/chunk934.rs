//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 934/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk934<F: Float>(t1563: F, t9: F, t1504: F, t967: F, t155: F, t506: F, t2911: F, t2913: F, t2873: F, t481: F, t1533: F, t133: F, t8146: F) -> (F, F, F, F, F, F) {
    let t8231 = t9 * t1563;
    let t8232 = t967 * t1504;
    let t8236 = t155 * t506;
    let t8238 = t2911 * t8236 * t2913;
    let t8240 = t2873 * t481;
    let t8244 = t967 * t1533;
    let t8249 = F::new(0.11495033333333333333e1) * t133 * t8146;
    (t8231, t8232, t8238, t8240, t8244, t8249)
}
