//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 766/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk766(t68422: f64, t68440: f64, t8667: f64, t21714: f64, t8830: f64, t14251: f64, t73692: f64, t15376: f64, t69568: f64, t68399: f64, t21709: f64, t68448: f64, t73727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73840 = t68440 * t68422 * t8667;
    let t73843 = t68440 * t21714 * t8830;
    let t73845 = t73692 * t14251;
    let t73847 = t69568 * t15376;
    let t73849 = 0.24829349937757072982e-4_f64 * t68399;
    let t73851 = t68448 * t21709 * t73727;
    (t73840, t73843, t73845, t73847, t73849, t73851)
}
