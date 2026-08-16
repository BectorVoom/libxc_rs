//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 772/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk772(t10628: f64, t2365: f64, t6111: f64, t10893: f64, t959: f64, t12709: f64, t10677: f64, t935: f64, t1445: f64, t813: f64, t2949: f64, t3234: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13118 = t2365 * t10628;
    let t13119 = t6111 * t13118;
    let t13120 = 0.59584149919750711116e-1_f64 * t13119;
    let t13121 = t10893 * t959;
    let t13124 = 0.19171462976960374838e1_f64 * t12709;
    let t13125 = t10677 * t935;
    let t13126 = t1445 * t13125;
    let t13127 = t813 * t13126;
    let t13129 = t2949 * t3234;
    (t13118, t13120, t13121, t13124, t13125, t13126, t13127, t13129)
}
