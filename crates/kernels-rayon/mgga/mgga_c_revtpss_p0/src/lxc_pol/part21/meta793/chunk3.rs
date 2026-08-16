//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2870/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2870(t2439: f64, t4628: f64, t1606: f64, t9303: f64, t52115: f64, t916: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41441: f64, t52112: f64) -> (f64, f64, f64, f64) {
    let t52126 = t2439 * t4628;
    let t52127 = 0.27595e0_f64 * t52126;
    let t52128 = t9303 * t1606;
    let t52130 = t916 * t52115;
    let t52134 = -0.60385000000000000002e0_f64 * t41365 + 0.20128333333333333334e0_f64 * t41367 + 0.60385000000000000002e0_f64 * t41308 - 0.40256666666666666667e0_f64 * t41330 - 0.26837777777777777778e0_f64 * t41332 + 0.10064166666666666667e0_f64 * t41334 + 0.11182407407407407408e0_f64 * t41336 - t52127 + 0.24528888888888888889e0_f64 * t52128 + 0.258925e1_f64 * t52130 - 0.543465e1_f64 * t52112 + 0.73586666666666666668e0_f64 * t41441;
    (t52126, t52128, t52130, t52134)
}
