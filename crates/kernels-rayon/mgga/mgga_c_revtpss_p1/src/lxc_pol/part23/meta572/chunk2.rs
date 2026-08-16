//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2169/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2169(t22767: f64, t23063: f64, t23077: f64, t23092: f64, t14312: f64, t18301: f64, t1522: f64, t18263: f64, t14328: f64, t14334: f64, t10552: f64, t10554: f64, t2403: f64, t4546: f64, t5962: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23094 = t22767 + t23063 + t23077 + t23092;
    let t23096 = 3.0_f64 * t14312;
    let t23097 = 3.0_f64 * t18301;
    let t23102 = 12.0_f64 * t18263 * t1522;
    let t23103 = 0.35089341735807877242e1_f64 * t14328;
    let t23104 = 0.17544670867903938621e1_f64 * t14334;
    let t23105 = 9.0_f64 * t2403 * t4546 * t5962 - t10552 + t10554 + t23096 + t23097 + t23102 + t23103 - t23104 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t23094, t23096, t23097, t23102, t23103, t23104, t23105)
}
