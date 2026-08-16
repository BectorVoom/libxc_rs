//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1127/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1127(t2601: f64, t4323: f64, t1005: f64, t1006: f64, t11003: f64, t10898: f64, t10913: f64, t6969: f64, t7183: f64, t9008: f64, t9037: f64, t7176: f64, t9264: f64) -> (f64, f64, f64, f64, f64) {
    let t11139 = t4323 * t2601;
    let t11140 = t11139 * t1005;
    let t11149 = t11003 * t1006;
    let t11159 = -t7183 + 0.22831111111111111111e-1_f64 * t6969 + 0.45662222222222222221e-1_f64 * t9008 - t9037 - 0.17123333333333333333e-1_f64 * t10898 + 0.5137e-1_f64 * t10913;
    let t11166 = -t7176 + 0.23744444444444444444e-1_f64 * t6969 + 0.47488888888888888888e-1_f64 * t9008 - t9264 - 0.17808333333333333333e-1_f64 * t10898 + 0.53425e-1_f64 * t10913;
    (t11139, t11140, t11149, t11159, t11166)
}
