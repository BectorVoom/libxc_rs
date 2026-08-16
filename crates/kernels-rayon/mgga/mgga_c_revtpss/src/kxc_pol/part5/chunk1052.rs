//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1052/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1052(t13878: f64, t3978: f64, t2619: f64, t5635: f64, t1398: f64, t1882: f64, t13848: f64, t3938: f64, t9818: f64, t9816: f64, t125: f64, t5658: f64) -> (f64, f64, f64, f64, f64) {
    let t13880 = 0.50820002809285328225e-3_f64 * t3978 * t13878;
    let t13887 = t5635 * t2619;
    let t13926 = t1882 * t1398;
    let t13941 = t9818 * t13848 * t3938;
    let t13943 = 0.10164000561857065645e-3_f64 * t9816 * t13941;
    let t13944 = t125 * t5658;
    (t13880, t13887, t13926, t13943, t13944)
}
