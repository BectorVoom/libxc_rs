//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 635/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk635<F: Float>(t4352: F, t4957: F, t1758: F, t11: F, t1663: F, t418: F, t1407: F, t571: F, t1764: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4958 = t4957 * t4352;
    let t4959 = t1758 * t4958;
    let t4960 = t11 * t4959;
    let t4962 = t1663 * t418;
    let t4963 = t4962 * t1407;
    let t4964 = t1758 * t4963;
    let t4965 = t11 * t4964;
    let t4967 = t1663 * t4352;
    let t4968 = t571 * t4967;
    let t4969 = t11 * t4968;
    let t4971 = t1764 * t418;
    let t4972 = t4971 * t1407;
    (t4958, t4959, t4960, t4962, t4963, t4964, t4965, t4967, t4968, t4969, t4971, t4972)
}
