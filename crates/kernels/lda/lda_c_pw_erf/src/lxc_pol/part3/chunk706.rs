//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 706/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk706<F: Float>(t2058: F, t331: F, t2055: F, t1371: F, t4680: F, t3587: F, t4666: F, t4676: F, t4693: F, t589: F, t4689: F, t4659: F, t21: F, t2782: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4998 = 0.017777777777777778 * t331 * t2058;
    let t5000 = 0.002962962962962963 * t331 * t2055;
    let t5001 = t1371 * t4680;
    let t5004 = t3587 * t4666;
    let t5007 = t1371 * t4676;
    let t5010 = t589 * t4693;
    let t5013 = t589 * t4689;
    let t5017 = 0.015996296296296297 * t4659;
    let t5021 = t21 * t2782;
    (t4998, t5000, t5001, t5004, t5007, t5010, t5013, t5017, t5021)
}
