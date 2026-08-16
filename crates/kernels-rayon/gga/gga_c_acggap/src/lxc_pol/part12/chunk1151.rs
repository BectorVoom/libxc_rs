//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1151/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1151(t2407: f64, t469: f64, t301: f64, t694: f64, t2248: f64, t3952: f64, t11179: f64, t1679: f64, t467: f64, t103: f64, t10761: f64, t2254: f64, t2408: f64, t32276: f64, t33358: f64, t33383: f64, t33397: f64, t36647: f64, t5439: f64, t567: f64, t642: f64, t7288: f64, t7297: f64, t7301: f64, t8027: f64, t8040: f64, t8372: f64, t9096: f64, t9098: f64, t9121: f64, t922: f64, t9460: f64) -> f64 {
    let t36686 = t2407 * t469;
    let t36689 = 6.0_f64 * t694 * t36686 * t301;
    let t36706 = t2248 * t3952;
    let t36715 = 2.0_f64 * t1679 * t11179 * t467;
    let t36716 = t103 * t2407;
    let t36726 = 6.0_f64 * t2254 * t567 * t642 * t922 - 6.0_f64 * t10761 * t5439 * t7297 + 2.0_f64 * t2408 * t567 * t8027 + 12.0_f64 * t33358 * t7297 * t9460 + 2.0_f64 * t33383 * t9096 * t9460 - 12.0_f64 * t33397 * t8040 * t8372 - 3.0_f64 * t36647 * t7297 * t8040 + 4.0_f64 * t36706 * t9096 * t9098 + 6.0_f64 * t36716 * t567 * t7288 + 3.0_f64 * t567 * t7301 * t9121 - t32276 + t36689 - t36715;
    t36726
}
