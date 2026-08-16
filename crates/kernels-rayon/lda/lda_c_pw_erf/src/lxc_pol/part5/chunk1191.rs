//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1191/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1191(t4794: f64, t571: f64, t7418: f64, t1318: f64, t7719: f64, t3859: f64, t519: f64, t7651: f64, t3802: f64, t7691: f64, t1325: f64, t7687: f64) -> (f64, f64, f64, f64, f64) {
    let t21604 = t571 * t4794 * t7418;
    let t21605 = 8.0_f64 / 27.0_f64 * t21604;
    let t21607 = t1318 * t4794 * t7719;
    let t21608 = 16.0_f64 / 27.0_f64 * t21607;
    let t21610 = t519 * t3859 * t7651;
    let t21611 = 16.0_f64 / 45.0_f64 * t21610;
    let t21613 = t519 * t3802 * t7691;
    let t21614 = 8.0_f64 / 45.0_f64 * t21613;
    let t21616 = t1325 * t3859 * t7687;
    (t21605, t21608, t21611, t21614, t21616)
}
