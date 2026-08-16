//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 847/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk847(t11499: f64, t11557: f64, t3257: f64, t11773: f64, t3134: f64, t1133: f64, t3824: f64) -> (f64, f64, f64) {
    let t13363 = t3257 * t11499 * t11557;
    let t13367 = t11773 * t3134 / 32.0_f64;
    let t13368 = t3824 * t1133;
    (t13363, t13367, t13368)
}
