//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 756/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk756(t1023: f64, t35385: f64, t1381: f64, t3549: f64, t11699: f64, t747: f64, t3516: f64, t475: f64, t3529: f64, t2366: f64, t6508: f64, t172: f64, t2754: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35719 = t1023 * t35385;
    let t35770 = t3549 * t1381;
    let t35781 = t11699 * t747;
    let t35845 = t3516 * t475;
    let t35887 = t3529 * t475;
    let t35888 = t2366 * t35887;
    let t35893 = t6508 * t35887;
    let t35900 = t172 * t2754;
    (t35719, t35770, t35781, t35845, t35888, t35893, t35900)
}
