//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1170/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1170<F: Float>(t5: F, t114267: F, t114292: F, t114320: F, t114356: F, t117: F, t5883: F, t7724: F, t1936: F, t75941: F, t1518: F, t5876: F, t18245: F, t7741: F, t1501: F, t5920: F, t30138: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t114359 = piecewise3(t8, 0.0, t114267 + t114292 + t114320 + t114356);
    let t114360 = t114359 * t117;
    let t114363 = t7724 * t5883;
    let t114372 = 2.0 * t75941 * t1936;
    let t114373 = t5876 * t1518;
    let t114375 = 6.0 * t114373 * t1936;
    let t114377 = 6.0 * t18245 * t7741;
    let t114378 = t1501 * t5920;
    let t114380 = 6.0 * t114378 * t1936;
    let t114382 = 12.0 * t30138 * t7741;
    (t114360, t114363, t114372, t114373, t114375, t114377, t114378, t114380, t114382)
}
