//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 872/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk872(t5560: f64, t5790: f64, t7332: f64, t7434: f64, t7435: f64, t9178: f64, t9180: f64, t9185: f64, t9189: f64, t9192: f64, t9196: f64, t9200: f64) -> f64 {
    let t9388 = 0.15358125e0_f64 * t9178 + 0.3071625e0_f64 * t9180 - t5790 + 0.27385555555555555556e0_f64 * t5560 + 0.5477111111111111111e0_f64 * t7332 - t7434 - t7435 - 0.16431333333333333333e0_f64 * t9185 + 0.49294e0_f64 * t9189 - 0.16431333333333333333e0_f64 * t9192 + 0.24647e0_f64 * t9196 + 0.24647e0_f64 * t9200;
    t9388
}
