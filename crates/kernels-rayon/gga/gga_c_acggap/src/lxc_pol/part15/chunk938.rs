//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 938/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk938(t30886: f64, t30889: f64, t30904: f64, t30907: f64, t30920: f64, t30989: f64, t31001: f64, t31015: f64, t31020: f64, t31022: f64, t31036: f64, t31226: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32621 = 0.85748036236139473944e-3_f64 * t30886;
    let t32622 = 0.25724410870841842183e-2_f64 * t30889;
    let t32627 = 0.51448821741683684367e-2_f64 * t30904;
    let t32628 = 0.24009450146119052704e-1_f64 * t30907;
    let t32635 = 0.83861579438944405516e-2_f64 * t30920;
    let t32664 = 0.57165357490759649297e-2_f64 * t30989;
    let t32668 = 0.24009450146119052704e-1_f64 * t31001;
    let t32670 = 0.7145669686344956162e-3_f64 * t31015;
    let t32671 = 0.10482697429868050689e-2_f64 * t31020;
    let t32672 = 0.12004725073059526352e-1_f64 * t31022;
    let t32677 = 311.0_f64 / 432.0_f64 * t31036;
    let t32739 = 0.51448821741683684367e-2_f64 * t31226;
    (t32621, t32622, t32627, t32628, t32635, t32664, t32668, t32670, t32671, t32672, t32677, t32739)
}
