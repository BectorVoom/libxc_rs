//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1007/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1007(t11746: f64, t11751: f64, t11754: f64, t11755: f64, t11757: f64, t11762: f64, t11764: f64, t11766: f64, t11770: f64, t11773: f64, t11775: f64, t11779: f64, t11781: f64, t1268: f64, t2061: f64, t538: f64, t9772: f64, t9774: f64, t9782: f64, t9784: f64, t9786: f64, t9788: f64, t9806: f64) -> f64 {
    let t11790 = -0.07198333333333333_f64 * t11751 - t11754 - 0.14396666666666666_f64 * t11755 + 0.057777777777777775_f64 * t11757 + 0.08_f64 * t2061 * t1268 * t11746 - 0.02666666666666667_f64 * t11762 - 0.3466666666666667_f64 * t11764 - 0.24_f64 * t2061 * t538 * t11766 - 1.5836333333333332_f64 * t11770 - 1.2957_f64 * t11773 + 0.21595_f64 * t11775 + 0.8638_f64 * t11779 + 0.044444444444444446_f64 * t11781 + 0.05925925925925926_f64 * t9772 - 0.008888888888888889_f64 * t9774 - 0.022222222222222223_f64 * t9782 - 0.007407407407407408_f64 * t9784 + 0.0044444444444444444_f64 * t9786 + 0.0019753086419753087_f64 * t9788 - 0.07198333333333333_f64 * t9806;
    t11790
}
