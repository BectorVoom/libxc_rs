//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1563/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1563(t10722: f64, t857: f64, t10673: f64, t10676: f64, t10678: f64, t10682: f64, t10687: f64, t10692: f64, t10693: f64, t10700: f64, t10706: f64, t10711: f64, t10713: f64, t10717: f64, t10719: f64, t851: f64) -> (f64, f64) {
    let t10723 = t10722 * t857;
    let t10725 = t10673 - 0.42874018118069736972e-3_f64 * t10676 - 0.91464571985215438873e-3_f64 * t10678 + 0.85748036236139473944e-4_f64 * t10682 - t10687 + t10692 - 0.60023625365297631762e-1_f64 * t10693 - 0.25724410870841842183e-1_f64 * t851 * t10700 + 0.76230004213927992338e-3_f64 * t10706 + 0.21437009059034868486e-4_f64 * t10711 + 0.12004725073059526352e-1_f64 * t10713 + 0.16262400898971305032e-2_f64 * t10717 - 0.22866142996303859718e-3_f64 * t10719 - 0.68026775414003982663e-1_f64 * t10723;
    (t10723, t10725)
}
