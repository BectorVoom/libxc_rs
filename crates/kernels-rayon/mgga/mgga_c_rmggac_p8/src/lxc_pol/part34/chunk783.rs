//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 783/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk783(t15399: f64, t69598: f64, t21714: f64, t68440: f64, t9117: f64, t3148: f64, t3151: f64, t38354: f64, t21713: f64, t68651: f64, t9183: f64, t14025: f64, t35154: f64) -> (f64, f64, f64, f64, f64) {
    let t74107 = t69598 * t15399;
    let t74112 = t68440 * t21714 * t9117;
    let t74115 = t38354 * t3148 * t3151;
    let t74118 = t21713 * t68651 * t9183;
    let t74120 = t14025 * t35154;
    (t74107, t74112, t74115, t74118, t74120)
}
