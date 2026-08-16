//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1211/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1211(t5: f64, t115291: f64, t115305: f64, t115324: f64, t115348: f64, t117: f64, t5883: f64, t7968: f64, t110110: f64, t114373: f64, t114378: f64, t114385: f64, t114905: f64, t1312: f64, t1518: f64, t18245: f64, t2055: f64, t22633: f64, t28653: f64, t30138: f64, t30143: f64, t30570: f64, t34251: f64, t4248: f64, t5920: f64, t7359: f64, t75941: f64, t7889: f64, t7983: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t115351 = piecewise3(t8, 0.0_f64, t115291 + t115305 + t115324 + t115348);
    let t115352 = t115351 * t117;
    let t115358 = t7968 * t5883;
    let t115386 = 6.0_f64 * t110110 * t1518 + 6.0_f64 * t114373 * t2055 + 6.0_f64 * t114378 * t2055 + 2.0_f64 * t114385 * t2055 + 2.0_f64 * t114905 * t1312 + 6.0_f64 * t18245 * t7983 + 2.0_f64 * t2055 * t75941 + 2.0_f64 * t22633 * t7359 + 6.0_f64 * t28653 * t5920 + 12.0_f64 * t30138 * t7983 + 6.0_f64 * t30143 * t7983 + 6.0_f64 * t30570 * t4248 + 6.0_f64 * t30570 * t7889 + 6.0_f64 * t34251 * t5920 + t115352 + 6.0_f64 * t115358;
    (t115352, t115358, t115386)
}
