//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1007/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1007<F: Float>(t11746: F, t11751: F, t11754: F, t11755: F, t11757: F, t11762: F, t11764: F, t11766: F, t11770: F, t11773: F, t11775: F, t11779: F, t11781: F, t1268: F, t2061: F, t538: F, t9772: F, t9774: F, t9782: F, t9784: F, t9786: F, t9788: F, t9806: F) -> F {
    let t11790 = -F::cast_from(0.07198333333333333_f64) * t11751 - t11754 - F::cast_from(0.14396666666666666_f64) * t11755 + F::cast_from(0.057777777777777775_f64) * t11757 + F::cast_from(0.08_f64) * t2061 * t1268 * t11746 - F::cast_from(0.02666666666666667_f64) * t11762 - F::cast_from(0.3466666666666667_f64) * t11764 - F::cast_from(0.24_f64) * t2061 * t538 * t11766 - F::cast_from(1.5836333333333332_f64) * t11770 - F::cast_from(1.2957_f64) * t11773 + F::cast_from(0.21595_f64) * t11775 + F::cast_from(0.8638_f64) * t11779 + F::cast_from(0.044444444444444446_f64) * t11781 + F::cast_from(0.05925925925925926_f64) * t9772 - F::cast_from(0.008888888888888889_f64) * t9774 - F::cast_from(0.022222222222222223_f64) * t9782 - F::cast_from(0.007407407407407408_f64) * t9784 + F::cast_from(0.0044444444444444444_f64) * t9786 + F::cast_from(0.0019753086419753087_f64) * t9788 - F::cast_from(0.07198333333333333_f64) * t9806;
    t11790
}
