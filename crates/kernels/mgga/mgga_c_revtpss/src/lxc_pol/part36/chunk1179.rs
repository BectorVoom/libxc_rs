//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1179/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1179<F: Float>(t22738: F, t76: F, t38: F, t85037: F, t1936: F, t75941: F, t1518: F, t5876: F, t18245: F, t7741: F, t1501: F, t5920: F, t30138: F, t30004: F, t4248: F, t22633: F, t93: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t114343 = t76 * t22738;
    let t114349 = t85037 * t38;
    let t114372 = 2.0 * t75941 * t1936;
    let t114373 = t5876 * t1518;
    let t114375 = 6.0 * t114373 * t1936;
    let t114377 = 6.0 * t18245 * t7741;
    let t114378 = t1501 * t5920;
    let t114380 = 6.0 * t114378 * t1936;
    let t114382 = 12.0 * t30138 * t7741;
    let t114384 = 6.0 * t4248 * t30004;
    let t114385 = t93 * t22633;
    (t114343, t114349, t114372, t114373, t114375, t114377, t114378, t114380, t114382, t114384, t114385)
}
