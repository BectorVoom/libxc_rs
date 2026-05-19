//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1086/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1086<F: Float>(t10463: F, t1325: F, t2006: F, t2171: F, t3953: F, t4026: F, t835: F, t1896: F, t603: F, t12681: F, t12683: F, t12685: F, t12689: F, t12691: F, t12694: F, t12698: F, t12702: F, t12706: F) -> (F, F, F, F) {
    let t12708 = t1325 * t10463 * t2006;
    let t12709 = F::new(16.0) / F::new(135.0) * t12708;
    let t12711 = F::new(4.0) / F::new(9.0) * t2171 * t3953;
    let t12713 = F::new(2.0) / F::new(15.0) * t4026 * t835;
    let t12714 = t1896 * t603;
    let t12716 = t12681 + t12683 - t12685 - t12689 - t12691 - t12694 - t12698 + t12702 + t12706 - t12709 + t12711 - t12713 + F::cast_from(0.0011033703703703704_f64) * t12714;
    (t12709, t12711, t12713, t12716)
}
