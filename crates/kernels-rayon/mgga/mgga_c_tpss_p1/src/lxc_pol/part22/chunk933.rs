//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 933/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk933(t355: f64, t9036: f64, t215: f64, t334: f64, t68: f64, t333: f64, t219: f64, t2769: f64, t979: f64, t73: f64, t8549: f64, t8552: f64) -> (f64, f64, f64, f64, f64) {
    let t9038 = t355 * t9036 / 10368.0_f64;
    let t9040 = t215 * t68 * t334;
    let t9042 = 5.0_f64 / 1296.0_f64 * t333 * t9040;
    let t9058 = t2769 * t219;
    let t9065 = t979 * t979;
    let t9066 = 1.0_f64 / t9065;
    let t9067 = t73 * t9066;
    let t9076 = t8549 * t8552;
    (t9038, t9042, t9058, t9067, t9076)
}
