//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 933/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk933<F: Float>(t3220: F, t9381: F, t1112: F, t2079: F, t904: F, t820: F, t875: F, t2306: F, t2190: F, t3219: F, t3235: F, t2345: F, t3240: F, t3140: F, t9375: F, t2494: F, t6: F) -> (F, F, F, F, F, F, F, F) {
    let t9382 = t3220 * t9381;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9387 = t875 * t820;
    let t9388 = t2306 * t9387;
    let t9389 = t9386 * t9388;
    let t9393 = t3235 * t3219 * t2190;
    let t9397 = t2345 * t3240 * t2190;
    let t9401 = t3235 * t9375 * t3140;
    let t9404 = t6 * t2494;
    (t9382, t9385, t9388, t9389, t9393, t9397, t9401, t9404)
}
