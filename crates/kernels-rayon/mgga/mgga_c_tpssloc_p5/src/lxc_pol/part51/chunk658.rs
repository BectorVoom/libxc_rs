//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 658/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk658(t3870: f64, t5308: f64, t820: f64, t1367: f64, t5187: f64, t1341: f64, t1363: f64, t1831: f64, t3781: f64, t3783: f64, t3800: f64, t3803: f64, t3864: f64, t3867: f64, t5259: f64, t5289: f64, t5293: f64, t5303: f64, t5306: f64) -> (f64, f64, f64) {
    let t5310 = t3870 * t820 * t5308;
    let t5314 = t1367 * t820 * t5187;
    let t5317 = t3803 * t5259 / 768.0_f64 - t1341 * t5289 / 3072.0_f64 - t3803 * t5293 / 3072.0_f64 - 7.0_f64 / 4608.0_f64 * t3781 + 7.0_f64 / 4608.0_f64 * t3800 + t3864 + 7.0_f64 / 1152.0_f64 * t3867 - t3783 * t1831 / 768.0_f64 + t3803 * t5303 / 768.0_f64 + 7.0_f64 / 1152.0_f64 * t5306 + 5.0_f64 / 768.0_f64 * t1363 * t5310 - t1363 * t5314 / 768.0_f64;
    (t5310, t5314, t5317)
}
