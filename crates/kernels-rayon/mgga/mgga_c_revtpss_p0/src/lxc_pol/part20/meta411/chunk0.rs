//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1521/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1521(t3154: f64, t42871: f64, t1036: f64, t11240: f64, t42646: f64, t11268: f64, t3173: f64, t1063: f64, t11232: f64, t3172: f64, t11982: f64, t11285: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t42872 = t3154 * t3154;
    let t42873 = t42871 * t42872;
    let t42879 = t11240 * t1036 * t42646;
    let t42883 = t11268 * t3173;
    let t42886 = t1063 * t3172 * t11232;
    let t42889 = t1063 * t3172 * t11982;
    let t42892 = t3127 * t3172 * t11285;
    (t42872, t42873, t42879, t42883, t42886, t42889, t42892)
}
