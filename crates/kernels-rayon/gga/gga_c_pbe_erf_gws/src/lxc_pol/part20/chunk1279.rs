//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1279/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1279(t15362: f64, t9270: f64, t53841: f64, t53923: f64, t9942: f64, t11354: f64, t14797: f64, t3989: f64, t3990: f64, t15134: f64, t51563: f64, t1161: f64, t274: f64) -> (f64, f64, f64, f64, f64) {
    let t56228 = t9270 * t15362;
    let t56236 = t53923 * t53841 * t9942;
    let t56240 = t3989 * t3990 * t14797 * t11354;
    let t56242 = t51563 * t15134;
    let t56246 = t274 * t1161;
    (t56228, t56236, t56240, t56242, t56246)
}
