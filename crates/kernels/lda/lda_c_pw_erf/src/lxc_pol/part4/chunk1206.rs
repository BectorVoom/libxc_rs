//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1206/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1206<F: Float>(t10015: F, t6763: F, t6767: F, t12475: F, t4479: F, t4849: F, t4633: F, t6762: F, t4620: F, t6766: F, t17806: F, t17807: F, t17808: F, t17809: F, t17810: F, t17811: F, t17812: F, t17813: F, t17814: F, t17816: F, t17820: F, t17823: F) -> (F, F, F, F, F, F) {
    let t17825 = 64.0 / 45.0 * t10015 * t6763;
    let t17827 = 32.0 / 27.0 * t10015 * t6767;
    let t17830 = 64.0 / 45.0 * t12475 * t4479 * t4849;
    let t17833 = 128.0 / 45.0 * t12475 * t6762 * t4633;
    let t17836 = 64.0 / 27.0 * t12475 * t6766 * t4620;
    let t17837 = t17806 - t17807 - t17808 + t17809 + t17810 + t17811 - t17812 - t17813 + 0.21642082724729686 * t17814 + 0.07214027574909895 * t17816 + 0.21642082724729686 * t17820 - t17823 - t17825 + t17827 - t17830 - t17833 + t17836;
    (t17825, t17827, t17830, t17833, t17836, t17837)
}
