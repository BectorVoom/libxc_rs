//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1261/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1261(t31702: f64, t31704: f64, t32915: f64, t36070: f64, t36072: f64, t36075: f64, t36083: f64, t36115: f64, t36119: f64, t36123: f64, t37837: f64, t37848: f64, t37857: f64, t37859: f64, t37860: f64, t37861: f64, t37864: f64, t40418: f64) -> f64 {
    let t42097 = -t37837 - t37848 + t36070 - t36072 + 0.31448092289604152069e-3_f64 * t31702 + 0.41930789719472202758e-3_f64 * t31704 + t36075 + t37857 - t32915 + 0.85748036236139473944e-3_f64 * t36083 + 0.37737710747524982482e-1_f64 * t40418 + t37859 + t37860 - t37861 + t37864 - 0.85748036236139473944e-3_f64 * t36115 + 0.83861579438944405516e-3_f64 * t36119 - 0.12579236915841660827e-2_f64 * t36123;
    t42097
}
