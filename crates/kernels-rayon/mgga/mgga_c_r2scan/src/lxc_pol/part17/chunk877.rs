//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 877/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk877(t44: f64, t2892: f64, t788: f64, t5095: f64, t785: f64, t3190: f64, t560: f64, t551: f64, t552: f64, t1217: f64, t3000: f64, t3003: f64, t415: f64, t8571: f64, t903: f64, t99: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t9333 = t788 * t2892;
    let t9335 = t5095 * t785 * t9333;
    let t9337 = t3190 * t560;
    let t9339 = t551 * t552 * t9337;
    let t9353 = piecewise3(t45, 0.0_f64, -10.0_f64 / 27.0_f64 * t3000 * t415 + 40.0_f64 / 9.0_f64 * t903 * t1217 + 10.0_f64 / 9.0_f64 * t3003 * t415 + 5.0_f64 / 3.0_f64 * t99 * t8571);
    (t9335, t9339, t9353)
}
