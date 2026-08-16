//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1151/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1151(t13846: f64, t1841: f64, t2536: f64, t734: f64, t42931: f64, t42934: f64, t42937: f64, t42940: f64, t42943: f64, t42948: f64, t42951: f64, t42954: f64, t42956: f64, t42961: f64) -> f64 {
    let t47587 = t1841 * t2536 * t13846 * t734;
    let t47592 = -0.85450291446024714263e-3_f64 * t47587 - 0.32043859292259267849e-3_f64 * t42931 - t42934 - t42937 - t42940 + t42943 + t42948 - 0.96131577876777803547e-3_f64 * t42951 - t42954 + 0.64087718584518535698e-3_f64 * t42956 - t42961;
    t47592
}
