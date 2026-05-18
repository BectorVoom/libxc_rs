//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1211/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1211<F: Float>(t5: F, t115291: F, t115305: F, t115324: F, t115348: F, t117: F, t5883: F, t7968: F, t110110: F, t114373: F, t114378: F, t114385: F, t114905: F, t1312: F, t1518: F, t18245: F, t2055: F, t22633: F, t28653: F, t30138: F, t30143: F, t30570: F, t34251: F, t4248: F, t5920: F, t7359: F, t75941: F, t7889: F, t7983: F) -> (F, F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t115351 = piecewise3::<f64>(t8, F::new(0.0), t115291 + t115305 + t115324 + t115348);
    let t115352 = t115351 * t117;
    let t115358 = t7968 * t5883;
    let t115386 = F::new(6.0) * t110110 * t1518 + F::new(6.0) * t114373 * t2055 + F::new(6.0) * t114378 * t2055 + F::new(2.0) * t114385 * t2055 + F::new(2.0) * t114905 * t1312 + F::new(6.0) * t18245 * t7983 + F::new(2.0) * t2055 * t75941 + F::new(2.0) * t22633 * t7359 + F::new(6.0) * t28653 * t5920 + F::new(12.0) * t30138 * t7983 + F::new(6.0) * t30143 * t7983 + F::new(6.0) * t30570 * t4248 + F::new(6.0) * t30570 * t7889 + F::new(6.0) * t34251 * t5920 + t115352 + F::new(6.0) * t115358;
    (t115352, t115358, t115386)
}
