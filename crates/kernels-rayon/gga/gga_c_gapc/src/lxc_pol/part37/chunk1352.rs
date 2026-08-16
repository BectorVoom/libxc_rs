//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1352/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1352(t13281: f64, t1617: f64, t3808: f64, t2967: f64, t31767: f64, t2822: f64, t3832: f64, t7063: f64, t10529: f64, t8613: f64, t24915: f64, t3568: f64) -> (f64, f64, f64, f64, f64) {
    let t36288 = 24.0_f64 * t13281 * t3808 * t1617;
    let t36290 = 4.0_f64 * t31767 * t2967;
    let t36293 = 6.0_f64 * t7063 * t3832 * t2822;
    let t36295 = 4.0_f64 * t10529 * t8613;
    let t36297 = 4.0_f64 * t24915 * t3568;
    (t36288, t36290, t36293, t36295, t36297)
}
