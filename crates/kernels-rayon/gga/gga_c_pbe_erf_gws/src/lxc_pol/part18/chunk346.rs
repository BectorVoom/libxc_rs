//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 346/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk346(t1027: f64, t625: f64, t11: f64, t624: f64, t203: f64, t184: f64) -> (f64, f64, f64, f64, f64) {
    let t1028 = t625 * t1027;
    let t1029 = t11 * t1028;
    let t1031 = t624 + 0.18891666666666666667e-2_f64 * t1029;
    let t1032 = t203 * t1031;
    let t1033 = t1032 * t184;
    (t1028, t1029, t1031, t1032, t1033)
}
