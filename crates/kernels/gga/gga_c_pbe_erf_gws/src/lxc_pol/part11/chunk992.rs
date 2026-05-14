//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 992/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk992<F: Float>(t18280: F, t47902: F, t47904: F, t47906: F, t47910: F, t47914: F, t47916: F, t47918: F, t47920: F, t47922: F, t47926: F, t41245: F, t47372: F, t626: F, t11: F, t625: F) -> (F, F, F, F) {
    let t47927 = -t47902 - t47904 - t47906 + t18280 - t47910 + t47914 + t47916 - t47918 - t47920 - t47922 - t47926;
    let t47928 = 64.0 / 45.0 * t41245;
    let t47929 = t626 * t47372;
    let t47931 = t11 * t625 * t47929;
    (t47927, t47928, t47929, t47931)
}
