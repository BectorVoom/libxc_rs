//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 879/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk879(t5560: f64, t5859: f64, t7332: f64, t7387: f64, t7390: f64, t9178: f64, t9180: f64, t9185: f64, t9189: f64, t9192: f64, t9196: f64, t9200: f64) -> f64 {
    let t9492 = 0.31558125e0_f64 * t9178 + 0.6311625e0_f64 * t9180 - t5859 + 0.34731666666666666666e0_f64 * t5560 + 0.69463333333333333333e0_f64 * t7332 - t7387 - t7390 - 0.20839e0_f64 * t9185 + 0.62517e0_f64 * t9189 - 0.20839e0_f64 * t9192 + 0.312585e0_f64 * t9196 + 0.312585e0_f64 * t9200;
    t9492
}
