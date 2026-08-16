//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1159/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1159(t12345: f64, t3876: f64, t22843: f64, t241: f64, t67: f64, t3872: f64, t12353: f64, t3866: f64, t12339: f64, t12211: f64, t12375: f64, t12012: f64, t12215: f64, t12240: f64, t12305: f64, t12336: f64, t12368: f64, t1328: f64, t1363: f64, t210: f64, t3719: f64, t3733: f64, t3765: f64, t3783: f64, t3870: f64, t39622: f64, t40026: f64, t5246: f64, t5248: f64, t820: f64) -> f64 {
    let t40065 = t12345 * t3876;
    let t40070 = t241 * t22843 * t67;
    let t40079 = t12345 * t3872;
    let t40081 = t3866 * t12353;
    let t40083 = t12339 * t3872;
    let t40089 = t12211 * t12375;
    let t40101 = 5.0_f64 / 128.0_f64 * t12336 * t3872 - 119.0_f64 / 576.0_f64 * t40065 - 5.0_f64 / 32.0_f64 * t3783 * t12353 + 35.0_f64 / 128.0_f64 * t1363 * t40070 * t820 * t40026 + 5.0_f64 / 256.0_f64 * t1363 * t3870 * t820 * t39622 + 595.0_f64 / 576.0_f64 * t40079 + 35.0_f64 / 48.0_f64 * t40081 - 35.0_f64 / 96.0_f64 * t40083 + 3.0_f64 / 256.0_f64 * t5246 * t5248 * t12368 * t12240 - 7.0_f64 / 4.0_f64 * t40089 - 3.0_f64 / 2.0_f64 * t12215 * t210 * t3765 * t3719 + t3733 * t210 * t1328 * t12012 / 4.0_f64 + 5.0_f64 / 64.0_f64 * t3783 * t12305;
    t40101
}
