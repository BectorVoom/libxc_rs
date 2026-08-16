//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1158/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1158(t27972: f64, t8707: f64, t32275: f64, t33943: f64, t32279: f64, t125: f64, t246: f64, t32276: f64, t551: f64, t5774: f64, t32292: f64, t33959: f64) -> (f64, f64, f64, f64) {
    let t125861 = t8707 * t27972;
    let t125867 = t33943 * t32275;
    let t125868 = t125867 * t32279;
    let t125873 = t32276 * t551 * t246 * t125 * t5774;
    let t125875 = t33959 * t32292;
    (t125861, t125868, t125873, t125875)
}
