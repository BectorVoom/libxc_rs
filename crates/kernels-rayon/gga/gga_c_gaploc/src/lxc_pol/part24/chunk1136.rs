//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1136/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1136(t30258: f64, t587: f64, t912: f64, t21071: f64, t901: f64, t4379: f64, t9573: f64, t1402: f64, t1429: f64, t3162: f64, t20237: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t30260 = t587 * t912 * t30258;
    let t30261 = 0.38342925953920749676e0_f64 * t30260;
    let t30263 = 0.29792074959875355558e-1_f64 * t21071 * t901;
    let t30265 = 0.59584149919750711116e-1_f64 * t4379 * t9573;
    let t30288 = 0.17875244975925213335e0_f64 * t1429 * t1402 * t3162;
    let t30292 = t544 * t20237;
    (t30261, t30263, t30265, t30288, t30292)
}
