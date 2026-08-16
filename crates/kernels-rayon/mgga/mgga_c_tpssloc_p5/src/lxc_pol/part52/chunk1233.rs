//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1233/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1233(t33702: f64, t33722: f64, t33743: f64, t33759: f64, t3: f64, t1873: f64, t27921: f64, t24972: f64, t7769: f64, t7423: f64, t7467: f64, t1458: f64, t31937: f64, t33177: f64, t33179: f64, t33181: f64, t33184: f64, t33187: f64, t33190: f64, t33192: f64, t33195: f64, t577: f64, t8508: f64) -> (f64, f64, f64) {
    let t33761 = t33702 + t33722 + t33743 + t33759;
    let t33762 = t3 * t33761;
    let t33774 = t27921 * t1873;
    let t33776 = t24972 * t7769;
    let t33778 = t7423 * t7467;
    let t33783 = 0.45e1_f64 * t33761 * t577 + 0.135e2_f64 * t31937 * t1458 + 0.135e2_f64 * t33774 + 27.0_f64 * t33776 + 0.135e2_f64 * t33778 + 0.135e2_f64 * t33177 + 27.0_f64 * t33179 + 0.135e2_f64 * t33181 + t33184 + t33187 + t33190 + t33192 + t33195 + t8508;
    (t33761, t33762, t33783)
}
