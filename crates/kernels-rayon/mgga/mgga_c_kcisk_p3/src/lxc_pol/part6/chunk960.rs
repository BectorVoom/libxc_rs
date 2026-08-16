//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 960/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk960(t11986: f64, t23947: f64, t23949: f64, t23951: f64, t23969: f64, t28768: f64, t28776: f64, t28780: f64, t28783: f64, t28790: f64, t28794: f64, t29759: f64, t7648: f64, t9163: f64) -> f64 {
    let t30003 = 0.34822083333333333333e-2_f64 * t28768 + 0.46429444444444444443e-2_f64 * t23947 - 0.12381185185185185185e-1_f64 * t23949 - 0.46429444444444444443e-2_f64 * t23951 + 0.27857666666666666666e-1_f64 * t28776 + 0.30952962962962962963e-2_f64 * t28780 + 0.51072388888888888887e-1_f64 * t28783 + 0.579e0_f64 * t7648 * t9163 - 0.43134342e-1_f64 * t11986 * t29759 + 0.69644166666666666665e-2_f64 * t23969 + 0.69644166666666666666e-2_f64 * t28790 + 0.18571777777777777778e-1_f64 * t28794;
    t30003
}
