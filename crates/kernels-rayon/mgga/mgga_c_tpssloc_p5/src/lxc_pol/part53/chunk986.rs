//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 986/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk986(t118833: f64, t22986: f64, t23270: f64, t31332: f64, t114770: f64, t1888: f64, t25045: f64, t33447: f64, t82159: f64, t33371: f64, t6547: f64, t31337: f64, t4119: f64) -> (f64, f64, f64, f64, f64) {
    let t121419 = t22986 * t23270 * t31332 * t118833;
    let t121426 = t1888 * t114770 * t25045;
    let t121429 = t22986 * t82159 * t33447;
    let t121431 = t6547 * t33371;
    let t121435 = t22986 * t23270 * t31337 * t4119;
    (t121419, t121426, t121429, t121431, t121435)
}
