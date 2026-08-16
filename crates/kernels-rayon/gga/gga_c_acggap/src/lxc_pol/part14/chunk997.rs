//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 997/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk997(t35258: f64, t7433: f64, t8739: f64, t1089: f64, t2079: f64, t535: f64, t7542: f64, t1967: f64, t8978: f64, t33953: f64, t5127: f64, t13287: f64, t31057: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35259 = 0.16006300097412701803e-1_f64 * t35258;
    let t35260 = t7433 * t8739;
    let t35261 = 0.37737710747524982482e-2_f64 * t35260;
    let t35271 = t2079 * t1089 * t535 * t7542;
    let t35273 = t1967 * t8978;
    let t35274 = 0.25724410870841842184e-2_f64 * t35273;
    let t35284 = t33953 * t5127;
    let t35286 = t31057 * t13287 * t35284;
    (t35259, t35261, t35271, t35274, t35284, t35286)
}
