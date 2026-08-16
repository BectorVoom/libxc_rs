//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2822/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2822(t125: f64, t23244: f64, t1558: f64, t5962: f64, t10777: f64, t14671: f64, t14686: f64, t6017: f64, t10811: f64, t23293: f64, t1544: f64, t23327: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t76289 = t125 * t23244;
    let t76302 = t5962 * t1558;
    let t76313 = t10777 * t14686 * t14671 * t6017;
    let t76315 = t10811 * t23293;
    let t76321 = t1544 * t1558;
    let t76330 = t10811 * t23327;
    (t76289, t76302, t76313, t76315, t76321, t76330)
}
