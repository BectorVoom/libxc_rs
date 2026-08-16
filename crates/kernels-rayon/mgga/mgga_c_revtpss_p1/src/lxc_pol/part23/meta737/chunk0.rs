//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2512/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2512(t10565: f64, t1532: f64, t4398: f64, t9419: f64, t14362: f64, t9572: f64, t37: f64, t4391: f64, t14728: f64, t9775: f64, t1549: f64, t40861: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t50892 = t1532 * t10565;
    let t50893 = t4398 * t9419;
    let t50901 = t14362 * t9572;
    let t50903 = t37 * t4391;
    let t50939 = t9775 * t14728;
    let t50941 = t40861 * t1549;
    (t50892, t50893, t50901, t50903, t50939, t50941)
}
