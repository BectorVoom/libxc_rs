//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 781/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk781(t1383: f64, t169: f64, t766: f64, t770: f64, t289: f64, t4598: f64, t274: f64, t413: f64, t39: f64, t745: f64, t1553: f64, t2718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5717 = 0.15917832887339686635e0_f64 * t169 * t766 * t1383;
    let t5726 = t169 * t770 * t1383;
    let t5730 = 0.31835665774679373271e-1_f64 * t169 * t289 * t4598;
    let t5732 = 0.12798016258123051272e1_f64 * t413 * t274;
    let t5733 = t39 * t745;
    let t5749 = 0.65290666666666666667e0_f64 * t1553 * t2718;
    (t5717, t5726, t5730, t5732, t5733, t5749)
}
