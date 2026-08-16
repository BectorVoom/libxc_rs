//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 584/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk584(t449: f64, t556: f64, t879: f64, t316: f64, t545: f64, t862: f64, t865: f64, t1658: f64, t322: f64, t3892: f64, t557: f64, t181: f64, t315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5368 = t449 * t556 * t879;
    let t5369 = t316 * t5368;
    let t5371 = t862 * t545;
    let t5372 = t5371 * t865;
    let t5378 = t1658 * t322;
    let t5379 = t449 * t5378;
    let t5381 = 0.13170898365871023197e1_f64 * t316 * t5379;
    let t5382 = t3892 * t557;
    let t5384 = t315 * t181;
    (t5368, t5369, t5372, t5379, t5381, t5382, t5384)
}
