//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2467/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2467(t11280: f64, t3127: f64, t3172: f64, t11870: f64, t11922: f64, t3115: f64, t11631: f64, t3133: f64, t1086: f64, t11223: f64, t3090: f64, t11866: f64, t11923: f64) -> (f64, f64, f64, f64, f64) {
    let t43266 = t3127 * t3172 * t11280;
    let t43277 = t3115 * t11922 * t11870;
    let t43279 = t11631 * t3133;
    let t43285 = t11223 * t1086 * t3090;
    let t43288 = t11866 * t11923;
    (t43266, t43277, t43279, t43285, t43288)
}
