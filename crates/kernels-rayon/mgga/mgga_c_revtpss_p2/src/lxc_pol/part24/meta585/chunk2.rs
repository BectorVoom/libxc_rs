//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1820/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1820(t189: f64, t512: f64, t92011: f64, t48297: f64, t48304: f64, t48306: f64, t39989: f64, t47084: f64, t47086: f64, t47088: f64, t47092: f64, t91982: f64, t91983: f64, t91984: f64, t91985: f64) -> (f64, f64, f64, f64, f64) {
    let t92013 = t512 * t92011 * t189;
    let t92014 = 0.4101607543286562663e4_f64 * t48297;
    let t92015 = 0.65061487801810439052e-1_f64 * t48304;
    let t92016 = 0.19263893255070628431e1_f64 * t48306;
    let t92017 = -t91982 - t91983 - t91984 - t91985 + t92013 - t92014 - t47084 + t92015 + t92016 - t39989 - t47086 + t47088 + t47092;
    (t92013, t92014, t92015, t92016, t92017)
}
