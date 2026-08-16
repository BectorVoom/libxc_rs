//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1225/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1225(t32540: f64, t34547: f64, t34549: f64, t34561: f64, t34566: f64, t34575: f64, t34578: f64, t34582: f64, t34586: f64, t34590: f64, t37150: f64, t37158: f64, t37167: f64, t39382: f64, t39386: f64, t39389: f64, t39391: f64, t39393: f64) -> f64 {
    let t41623 = -0.68598428988911579156e-2_f64 * t34547 - 0.32012600194825403606e-1_f64 * t34549 + t37150 + 0.31448092289604152068e-2_f64 * t39382 - 0.47172138434406228102e-2_f64 * t39386 + 0.62896184579208304138e-3_f64 * t39389 + 0.37737710747524982484e-2_f64 * t34561 + 0.27439371595564631662e-1_f64 * t39391 + 0.12862205435420921092e-2_f64 * t39393 - t34566 + t37158 - t34575 - t32540 + 0.12579236915841660827e-1_f64 * t34578 - 0.5031694766336664331e-2_f64 * t34582 + 0.75475421495049964968e-2_f64 * t34586 - 0.34299214494455789578e-2_f64 * t34590 - t37167;
    t41623
}
