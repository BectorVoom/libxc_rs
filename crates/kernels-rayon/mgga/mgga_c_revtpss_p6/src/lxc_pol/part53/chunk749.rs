//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 749/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk749(t265: f64, t393: f64, t1544: f64, t1963: f64, t207: f64, t7782: f64, t1583: f64, t1940: f64, t198: f64, t2403: f64, t7091: f64, t892: f64, t1102: f64, t1699: f64, t336: f64, t5023: f64, t7181: f64, t7840: f64) -> (f64, f64) {
    let t394 = t265 < t393;
    let t7847 = t1963 * t1544;
    let t7850 = t207 * t7782;
    let t7855 = -t1583 * t1940 * t7091 + t198 * t7850 * t892 + 3.0_f64 * t2403 * t7847;
    let t7856 = piecewise3(t394, t1102 * t198 * t336 * t7840 - t1699 * t5023 * t7181, t7855);
    (t7855, t7856)
}
