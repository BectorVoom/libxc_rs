//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1276/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1276(t10761: f64, t11179: f64, t19418: f64, t2249: f64, t2408: f64, t24794: f64, t24811: f64, t36769: f64, t36771: f64, t36774: f64, t38534: f64, t38540: f64, t40955: f64, t5399: f64, t5439: f64, t567: f64, t643: f64, t6614: f64, t7297: f64, t8040: f64, t8372: f64, t9096: f64, t9460: f64, t9476: f64) -> f64 {
    let t42366 = -6.0_f64 * t10761 * t7297 * t9476 - 6.0_f64 * t11179 * t5439 * t7297 - t19418 * t567 * t643 - t2249 * t567 * t6614 - 2.0_f64 * t2408 * t5399 * t567 + 2.0_f64 * t24794 * t9096 * t9460 - 3.0_f64 * t24811 * t7297 * t8040 + 6.0_f64 * t38534 * t7297 * t9460 + 4.0_f64 * t38540 * t9096 * t9460 - 12.0_f64 * t40955 * t8040 * t8372 - t36769 + t36771 + t36774;
    t42366
}
