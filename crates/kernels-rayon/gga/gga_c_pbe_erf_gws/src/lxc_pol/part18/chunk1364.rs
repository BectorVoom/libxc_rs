//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1364/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1364(t1173: f64, t12166: f64, t3824: f64, t898: f64, t14682: f64, t3989: f64, t50912: f64, t15159: f64, t3111: f64, t833: f64, t850: f64, t13796: f64, t13798: f64) -> (f64, f64, f64, f64) {
    let t57449 = t1173 * t12166;
    let t57451 = t898 * t3824;
    let t57454 = t3989 * t14682 * t57451 * t50912;
    let t57458 = t850 * t3111 * t15159 * t833;
    let t57462 = t3989 * t13796 * t57451 * t13798;
    (t57449, t57454, t57458, t57462)
}
