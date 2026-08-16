//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2084/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2084(t23562: f64, t343: f64, t82916: f64, t3008: f64, t40: f64, t23482: f64, t3: f64, t23563: f64, t23514: f64, t3128: f64, t82895: f64, t23471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t82918 = t23562 * t82916 * t343;
    let t82921 = t40 * t3008;
    let t82923 = t23562 * t82921 * t343;
    let t82926 = t23482 * t3;
    let t82927 = t82926 * t23563;
    let t82941 = t82895 * t3128 * t23514;
    let t82943 = t23482 * t23471;
    (t82918, t82921, t82923, t82926, t82927, t82941, t82943)
}
