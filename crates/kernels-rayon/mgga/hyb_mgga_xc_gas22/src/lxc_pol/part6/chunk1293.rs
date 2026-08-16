//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1293/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1293(t1238: f64, t2024: f64, t2027: f64, t23804: f64, t23975: f64, t23977: f64, t23985: f64, t23987: f64, t23990: f64, t23992: f64, t23994: f64, t23996: f64, t23999: f64, t24455: f64, t28046: f64, t28049: f64, t28057: f64, t28060: f64, t28066: f64, t3: f64, t3150: f64, t3925: f64, t6457: f64, t684: f64, t687: f64, t8492: f64) -> f64 {
    let t28084 = -t28046 / 32.0_f64 - t28049 / 32.0_f64 - 5.0_f64 / 144.0_f64 * t23975 + t23977 / 24.0_f64 + t23985 / 48.0_f64 + t23987 / 24.0_f64 - 5.0_f64 / 144.0_f64 * t23990 - t23992 / 32.0_f64 + t28057 / 96.0_f64 + t28060 / 96.0_f64 - t23994 / 32.0_f64 - t23996 / 16.0_f64 + t23999 / 24.0_f64 + t28066 / 216.0_f64 - t2024 * t2027 * t6457 * t3925 / 48.0_f64 - t684 * t687 * t23804 * t1238 / 32.0_f64 + t684 * t3150 * t8492 * t3 / 8.0_f64 - t684 * t687 * t24455 * t1238 / 32.0_f64;
    t28084
}
