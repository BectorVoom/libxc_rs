//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 771/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk771(t1250: f64, t482: f64, t5284: f64, t1042: f64, t1038: f64, t1802: f64, t1244: f64, t1241: f64) -> (f64, f64, f64, f64) {
    let t5286 = t482 * t5284 * t1250;
    let t5287 = t1042 * t5286;
    let t5291 = t1802 * t1038;
    let t5292 = t1244 * t5291;
    let t5293 = t1241 * t5292;
    (t5286, t5287, t5292, t5293)
}
