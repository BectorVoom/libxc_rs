//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2297/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2297(t476: f64, t52: f64, t475: f64, t467: f64, t1785: f64, t6594: f64, t12678: f64, t16706: f64, t20283: f64, t20285: f64, t20287: f64, t24230: f64, t24234: f64, t24238: f64, t24242: f64, t24246: f64, t24250: f64) -> (f64, f64, f64, f64, f64) {
    let t24677 = t476 * t476;
    let t24679 = 1.0_f64 / t52 / t24677;
    let t24680 = t475 * t24679;
    let t24681 = t467 * t24680;
    let t24684 = t1785 * t6594;
    let t24697 = -t12678 + 0.11111111111111111111e-1_f64 * t16706 + 0.55555555555555555555e-2_f64 * t20283 - 0.16666666666666666667e-1_f64 * t20285 - 0.83333333333333333334e-2_f64 * t20287 + 0.92592592592592592592e-2_f64 * t24230 - 0.33333333333333333333e-1_f64 * t24234 - 0.16666666666666666666e-1_f64 * t24238 + 0.50000000000000000001e-1_f64 * t24242 + 0.50000000000000000001e-1_f64 * t24246 + 0.83333333333333333333e-2_f64 * t24250;
    (t24679, t24680, t24681, t24684, t24697)
}
