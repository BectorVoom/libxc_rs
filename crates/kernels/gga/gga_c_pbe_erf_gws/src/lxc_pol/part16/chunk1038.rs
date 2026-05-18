//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1038/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1038<F: Float>(t3223: F, t9376: F, t2113: F, t274: F, t3221: F, t3220: F, t1112: F, t2079: F, t904: F, t820: F, t875: F, t2306: F) -> (F, F, F, F, F, F, F) {
    let t9377 = t9376 * t3223;
    let t9380 = t2113 * t274;
    let t9381 = t3221 * t9380;
    let t9382 = t3220 * t9381;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9387 = t875 * t820;
    let t9388 = t2306 * t9387;
    (t9377, t9380, t9381, t9382, t9385, t9386, t9388)
}
