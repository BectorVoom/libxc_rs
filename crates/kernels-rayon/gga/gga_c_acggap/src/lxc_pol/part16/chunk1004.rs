//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1004/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1004(t35475: f64, t1992: f64, t7585: f64, t7586: f64, t8906: f64, t1983: f64, t8402: f64, t30105: f64, t8897: f64, t30268: f64, t8783: f64, t1479: f64, t429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35476 = 0.7145669686344956162e-3_f64 * t35475;
    let t35479 = t7585 * t7586 * t1992 * t8906;
    let t35480 = 0.28582678745379824648e-3_f64 * t35479;
    let t35484 = t7585 * t7586 * t1983 * t8402;
    let t35485 = 0.14291339372689912324e-3_f64 * t35484;
    let t35486 = t30105 * t8897;
    let t35496 = t30268 * t8783;
    let t35497 = 0.94344276868812456204e-2_f64 * t35496;
    let t35500 = t429 * t1479;
    (t35476, t35480, t35485, t35486, t35497, t35500)
}
