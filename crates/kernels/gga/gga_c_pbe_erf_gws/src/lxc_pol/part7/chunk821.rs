//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 821/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk821<F: Float>(t16938: F, t16907: F, t16910: F, t16912: F, t16917: F, t16921: F, t16925: F, t16927: F, t16929: F, t16931: F, t16936: F, t155: F, t1660: F, t1665: F, t587: F, t5009: F, t5283: F) -> (F, F, F, F) {
    let t16939 = 64.0 / 15.0 * t16938;
    let t16940 = -t16907 - t16910 - t16912 - t16917 + t16921 + t16925 + t16927 - t16929 - t16931 - t16936 + t16939;
    let t16942 = t155 * t1660;
    let t16944 = t587 * t16942 * t1665;
    let t16945 = 16.0 / 81.0 * t16944;
    let t16947 = t587 * t5283 * t5009;
    (t16939, t16940, t16945, t16947)
}
