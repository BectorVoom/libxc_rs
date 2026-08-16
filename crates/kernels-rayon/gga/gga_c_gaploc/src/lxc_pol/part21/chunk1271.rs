//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1271/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1271(t33568: f64, t10847: f64, t22693: f64, t7572: f64, t24554: f64, t959: f64, t20671: f64, t22538: f64, t24549: f64, t11057: f64, t28737: f64, t10942: f64, t28673: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33569 = 0.29792074959875355558e-1_f64 * t33568;
    let t33572 = 0.18404604457881959845e2_f64 * t7572 * t22693 * t10847;
    let t33573 = t24554 * t959;
    let t33574 = 0.14896037479937677779e-1_f64 * t33573;
    let t33580 = t22538 * t20671 * t24549;
    let t33581 = 0.85206502119823888168e-1_f64 * t33580;
    let t33583 = t28737 * t11057;
    let t33584 = 0.76685851907841499352e0_f64 * t33583;
    let t33585 = t28673 * t10942;
    (t33569, t33572, t33574, t33581, t33584, t33585)
}
