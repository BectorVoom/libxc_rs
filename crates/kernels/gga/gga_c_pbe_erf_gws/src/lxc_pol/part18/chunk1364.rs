//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1364/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1364<F: Float>(t1173: F, t12166: F, t3824: F, t898: F, t14682: F, t3989: F, t50912: F, t15159: F, t3111: F, t833: F, t850: F, t13796: F, t13798: F) -> (F, F, F, F) {
    let t57449 = t1173 * t12166;
    let t57451 = t898 * t3824;
    let t57454 = t3989 * t14682 * t57451 * t50912;
    let t57458 = t850 * t3111 * t15159 * t833;
    let t57462 = t3989 * t13796 * t57451 * t13798;
    (t57449, t57454, t57458, t57462)
}
