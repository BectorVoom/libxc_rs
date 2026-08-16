//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1056/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1056(t100993: f64, t117390: f64, t120818: f64, t124587: f64, t12524: f64, t1458: f64, t20173: f64, t2039: f64, t24462: f64, t24465: f64, t27170: f64, t27273: f64, t27276: f64, t27281: f64, t31287: f64, t32295: f64, t33192: f64, t34099: f64, t3941: f64, t4072: f64, t55353: f64, t577: f64, t7056: f64, t7235: f64, t7801: f64, t7956: f64, t84033: f64, t8717: f64, t94170: f64) -> f64 {
    let t124668 = 54.0_f64 * t24465 * t27281 + 54.0_f64 * t94170 * t7235 + 0.135e2_f64 * t32295 * t4072 + 54.0_f64 * t20173 * t34099 + 54.0_f64 * t3941 * t7056 * t7801 + 54.0_f64 * t3941 * t2039 * t27170 + 0.135e2_f64 * t117390 * t1458 + t120818 + 54.0_f64 * t100993 * t7956 + 54.0_f64 * t84033 * t7956 + 27.0_f64 * t55353 * t8717 + 0.45e1_f64 * t124587 * t577 + 54.0_f64 * t24465 * t27273 + 54.0_f64 * t24465 * t27276 + t31287 + t33192 + 54.0_f64 * t12524 * t34099 + 27.0_f64 * t24462 * t7801;
    t124668
}
