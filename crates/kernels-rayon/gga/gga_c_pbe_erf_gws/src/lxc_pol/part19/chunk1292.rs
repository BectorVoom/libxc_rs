//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1292/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1292(t3733: f64, t54580: f64, t13781: f64, t3808: f64, t3972: f64, t52000: f64, t13782: f64, t3861: f64, t2306: f64, t3037: f64, t3975: f64, t9385: f64) -> (f64, f64, f64, f64) {
    let t56626 = t54580 * t3733;
    let t56638 = t3972 * t13781 * t3808 * t52000;
    let t56642 = t3972 * t13781 * t3861 * t13782;
    let t56647 = t3972 * t3975 * t9385 * t2306 * t3037;
    (t56626, t56638, t56642, t56647)
}
