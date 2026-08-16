//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 796/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk796(t32357: f64, t5539: f64, t9647: f64, t32436: f64, t2558: f64, t32743: f64, t7064: f64, t7069: f64, t8878: f64, t10657: f64, t871: f64, t33360: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42988 = t9647 * t5539 * t32357;
    let t42991 = t9647 * t5539 * t32436;
    let t43027 = t9647 * t32743 * t2558;
    let t43042 = t7064 * t8878 * t7069;
    let t43072 = t10657 * t871;
    let t43093 = t9647 * t33360 * t2558;
    (t42988, t42991, t43027, t43042, t43072, t43093)
}
