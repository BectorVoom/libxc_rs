//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 733/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk733(t1985: f64, t7799: f64, t606: f64, t7610: f64, t1994: f64, t599: f64, t839: f64, t142: f64, t2030: f64, t1131: f64, t604: f64, t2060: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7800 = t7799 * t1985;
    let t7802 = t7610 * t606;
    let t7805 = t7799 * t1994;
    let t7807 = t599 * t839;
    let t7808 = t142 * t7807;
    let t7809 = t2030 * t7808;
    let t7811 = t604 * t1131;
    let t7812 = t142 * t7811;
    let t7813 = t2060 * t7812;
    (t7800, t7802, t7805, t7807, t7808, t7809, t7811, t7812, t7813)
}
