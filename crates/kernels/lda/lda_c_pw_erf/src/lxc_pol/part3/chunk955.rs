//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 955/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk955<F: Float>(t1704: F, t2765: F, t411: F, t1063: F, t274: F, t169: F, t242: F, t2818: F, t1113: F, t1143: F, t2929: F, t703: F) -> (F, F, F, F, F) {
    let t10874 = t2765 * t1704 * t411;
    let t10878 = F::cast_from(6.399008129061525_f64) * t1063 * t274;
    let t10881 = F::cast_from(2.4210827305188265_f64) * t169 * t2818 * t242;
    let t10883 = t169 * t1113 * t1143;
    let t10886 = t169 * t703 * t2929;
    (t10874, t10878, t10881, t10883, t10886)
}
