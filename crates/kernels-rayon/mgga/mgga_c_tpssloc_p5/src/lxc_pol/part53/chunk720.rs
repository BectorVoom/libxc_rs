//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 720/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk720(t3: f64, t8811: f64, t2039: f64, t3941: f64, t577: f64, t7230: f64, t8508: f64, t8717: f64, t192: f64, t533: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t8812 = t3 * t8811;
    let t8822 = 0.45e1_f64 * t8811 * t577 + 27.0_f64 * t7230 * t2039 + 27.0_f64 * t3941 * t8717 + t8508;
    let t8944 = t192 * t533;
    let t9003 = t89 * t2039;
    (t8812, t8822, t8944, t9003)
}
