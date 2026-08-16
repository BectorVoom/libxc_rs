//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1790/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1790(t24677: f64, t467: f64, t475: f64, t484: f64, t52: f64, t6594: f64, t6601: f64, t71187: f64, t71192: f64, t83849: f64, t83851: f64, t83860: f64, t83863: f64, t83871: f64, t83891: f64, t83897: f64, rho1: f64) -> f64 {
    let t91303 = 0.18292914397043087775e-1_f64 * t83849 + 0.34299214494455789578e-2_f64 * t83851 - 0.19055119163586549765e-2_f64 * t83860 + 0.57165357490759649296e-3_f64 * t83863 - 0.22866142996303859719e-2_f64 * t83871 - 0.17149607247227894789e-2_f64 * t83891 - 0.22866142996303859719e-2_f64 * t83897 + 0.30488190661738479624e-2_f64 * t71187 - 0.28582678745379824648e-3_f64 * t71192 + 0.4425022116877321001e0_f64 * t467 * t475 / t52 / t24677 / rho1 * t484 + 0.43445671692977333464e-1_f64 * t6601 * t6594 * t484;
    t91303
}
