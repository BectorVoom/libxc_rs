//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 811/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk811<F: Float>(t3202: F, t840: F, t2376: F, t3306: F, t829: F, t830: F, t1105: F, t2395: F, t2370: F, t2494: F, t831: F, t2358: F, t3039: F, t1114: F, t4409: F, t1: F, t3360: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9701 = 7.0 / 144.0 * t840 * t3202;
    let t9707 = t2376 * t3306;
    let t9709 = t829 * t830 * t9707;
    let t9716 = t2395 * t1105;
    let t9718 = t2370 * t830 * t9716;
    let t9721 = t831 * t2494;
    let t9723 = t2370 * t830 * t9721;
    let t9726 = t3039 * t2358;
    let t9729 = t1114 * t4409;
    let t9762 = t3360 * t1;
    (t9701, t9707, t9709, t9716, t9718, t9721, t9723, t9726, t9729, t9762)
}
