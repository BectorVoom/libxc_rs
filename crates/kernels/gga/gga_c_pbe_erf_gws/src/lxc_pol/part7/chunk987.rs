//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 987/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk987<F: Float>(t2118: F, t4422: F, t2362: F, t822: F, t2367: F, t4419: F, t4386: F, t4388: F, t892: F, t2402: F, t353: F, t814: F, t8599: F, t6824: F, t9270: F, t328: F, t6045: F, t824: F) -> (F, F, F, F, F, F) {
    let t19817 = t2118 * t4422;
    let t19819 = t822 * t19817 * t2362;
    let t19821 = t2367 * t4419;
    let t19824 = t4386 * t892 * t4388;
    let t19829 = t8599 * t353 * t2402 * t814;
    let t19836 = t9270 * t6824;
    let t19839 = t824 * t328 * t6045;
    (t19819, t19821, t19824, t19829, t19836, t19839)
}
