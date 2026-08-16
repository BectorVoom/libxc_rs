//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1172/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1172(t1321: f64, t2061: f64, t2105: f64, t93: f64, t1334: f64, t2023: f64, t3509: f64, t600: f64, t3533: f64, t1333: f64, t2074: f64, t7594: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13136 = t1321 * t2061;
    let t13146 = t93 * t2105;
    let t13154 = t2023 * t1334;
    let t13157 = 4.0_f64 / 3.0_f64 * t600 * t3509;
    let t13159 = 2.0_f64 / 3.0_f64 * t600 * t3533;
    let t13161 = t7594 * t1333 * t2074;
    (t13136, t13146, t13154, t13157, t13159, t13161)
}
