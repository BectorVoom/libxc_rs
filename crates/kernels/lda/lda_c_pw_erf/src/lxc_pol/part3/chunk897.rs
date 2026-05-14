//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 897/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk897<F: Float>(t325: F, t4629: F, t2954: F, t4614: F, t11: F, t503: F, t2092: F, t933: F, t11746: F, t11751: F, t11754: F, t11755: F, t11757: F, t11762: F, t11764: F, t11766: F, t11770: F, t11773: F, t1268: F, t2061: F, t538: F, t9772: F, t9774: F, t9782: F, t9784: F, t9786: F, t9788: F, t9806: F) -> (F, F, F, F) {
    let t11775 = t325 * t4629;
    let t11777 = t4614 * t2954;
    let t11779 = t11 * t503 * t11777;
    let t11781 = t933 * t2092;
    let t11790 = -0.07198333333333333 * t11751 - t11754 - 0.14396666666666666 * t11755 + 0.057777777777777775 * t11757 + 0.08 * t2061 * t1268 * t11746 - 0.02666666666666667 * t11762 - 0.3466666666666667 * t11764 - 0.24 * t2061 * t538 * t11766 - 1.5836333333333332 * t11770 - 1.2957 * t11773 + 0.21595 * t11775 + 0.8638 * t11779 + 0.044444444444444446 * t11781 + 0.05925925925925926 * t9772 - 0.008888888888888889 * t9774 - 0.022222222222222223 * t9782 - 0.007407407407407408 * t9784 + 0.0044444444444444444 * t9786 + 0.0019753086419753087 * t9788 - 0.07198333333333333 * t9806;
    (t11775, t11777, t11779, t11790)
}
