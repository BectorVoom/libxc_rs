//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1252/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1252(t4127: f64, t4419: f64, t1133: f64, t2416: f64, t13796: f64, t2417: f64, t343: f64, t3989: f64, t1123: f64, t51989: f64, t833: f64, t850: f64) -> (f64, f64, f64) {
    let t53715 = t4127 * t4419;
    let t53717 = t2416 * t1133;
    let t53721 = t3989 * t13796 * t53717 * t343 * t2417;
    let t53725 = t850 * t1123 * t51989 * t833;
    (t53715, t53721, t53725)
}
