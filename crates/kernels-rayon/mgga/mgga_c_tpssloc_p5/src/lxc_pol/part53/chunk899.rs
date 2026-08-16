//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 899/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk899(t32211: f64, t32280: f64, t3: f64, t112: f64, t8811: f64, t2039: f64, t7056: f64, t12524: f64, t20173: f64, t24462: f64, t24465: f64, t31284: f64, t31287: f64, t3941: f64, t577: f64, t671: f64, t7230: f64, t7235: f64, t8508: f64, t8717: f64) -> (f64, f64, f64, f64, f64) {
    let t32281 = t32211 + t32280;
    let t32282 = t3 * t32281;
    let t32295 = t8811 * t112;
    let t32308 = t2039 * t7056;
    let t32311 = 0.45e1_f64 * t32281 * t577 + 0.135e2_f64 * t32295 * t671 + 27.0_f64 * t24462 * t2039 + 54.0_f64 * t24465 * t7235 + 27.0_f64 * t7230 * t7056 + 27.0_f64 * t12524 * t8717 + 27.0_f64 * t20173 * t8717 + 54.0_f64 * t3941 * t32308 + t31284 + t31287 + t8508;
    (t32281, t32282, t32295, t32308, t32311)
}
