//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 958/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk958(t1894: f64, t2746: f64, t1100: f64, t1954: f64, t7335: f64, t5522: f64, t5525: f64, t5745: f64, t7352: f64, t7357: f64, t228: f64, t5758: f64) -> (f64, f64, f64, f64, f64) {
    let t7493 = 1.0_f64 * t2746 * t1894;
    let t7494 = t1100 * t1954;
    let t7500 = 0.35616666666666666666e-1_f64 * t7335;
    let t7502 = -t5745 + 0.47488888888888888888e-1_f64 * t5522 - 0.17808333333333333333e-1_f64 * t5525 + 0.23744444444444444444e-1_f64 * t7357 - t7500 + 0.53425e-1_f64 * t7352;
    let t7504 = 0.621814e-1_f64 * t7502 * t228;
    let t7508 = 0.18541666666666666667e-1_f64 * t7335;
    let t7510 = -t5758 + 0.24722222222222222222e-1_f64 * t5522 - 0.92708333333333333333e-2_f64 * t5525 + 0.12361111111111111111e-1_f64 * t7357 - t7508 + 0.278125e-1_f64 * t7352;
    (t7493, t7494, t7502, t7504, t7510)
}
