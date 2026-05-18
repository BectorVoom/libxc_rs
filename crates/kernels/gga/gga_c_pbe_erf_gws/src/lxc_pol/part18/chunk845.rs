//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 845/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk845<F: Float>(t525: F, t8108: F, t1477: F, t991: F, t551: F, t1480: F, t1076: F, t169: F, t301: F, t366: F, t1500: F, t2858: F) -> (F, F, F, F) {
    let t8302 = t525 * t8108;
    let t8308 = t1477 * t991;
    let t8309 = t8308 * t551;
    let t8310 = t8309 * t1480;
    let t8314 = t169 * t366 * t1076 * t301;
    let t8318 = t1500 * t2858;
    (t8302, t8310, t8314, t8318)
}
