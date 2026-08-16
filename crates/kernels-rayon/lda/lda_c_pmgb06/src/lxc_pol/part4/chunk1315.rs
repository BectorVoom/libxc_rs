//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1315/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1315(t17287: f64, t1444: f64, t6756: f64, t17261: f64, t17262: f64, t17263: f64, t17264: f64, t17265: f64, t17266: f64, t17268: f64, t17272: f64, t17275: f64, t17279: f64, t17282: f64, t17284: f64, t17286: f64) -> (f64, f64, f64) {
    let t17288 = 4.0_f64 / 81.0_f64 * t17287;
    let t17290 = 2.0_f64 / 45.0_f64 * t1444 * t6756;
    let t17291 = -t17261 - t17262 - t17263 - t17264 - t17265 - t17266 + t17268 + t17272 + t17275 + t17279 - t17282 - t17284 - t17286 + t17288 - t17290;
    (t17288, t17290, t17291)
}
