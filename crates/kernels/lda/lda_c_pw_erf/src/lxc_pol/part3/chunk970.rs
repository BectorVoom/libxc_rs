//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 970/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk970<F: Float>(t1143: F, t1198: F, t2929: F, t458: F, t116: F, t1191: F, t731: F, t732: F, t2693: F, t2695: F, t726: F, t4291: F, t4299: F) -> (F, F, F, F, F) {
    let t11233 = t1198 * t1143;
    let t11236 = F::new(0.3350512821420176) * t458 * t2929;
    let t11250 = F::new(6.693920255418272) * t731 * t732 * t1191 * t116;
    let t11254 = t726 * t2693 * t2695;
    let t11256 = t4291 * t4299;
    (t11233, t11236, t11250, t11254, t11256)
}
