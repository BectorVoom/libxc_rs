//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 926/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk926<F: Float>(t285: F, t2872: F, t695: F, t1063: F, t274: F, t169: F, t242: F, t2818: F, t2929: F, t703: F, t699: F, t1102: F, t1143: F) -> (F, F, F, F, F, F) {
    let t10872 = F::cast_from(0.0011622696607154768_f64) * t695 * t2872 * t285;
    let t10878 = F::cast_from(6.399008129061525_f64) * t1063 * t274;
    let t10881 = F::cast_from(2.4210827305188265_f64) * t169 * t2818 * t242;
    let t10886 = t169 * t703 * t2929;
    let t10897 = F::cast_from(0.21223777183119583_f64) * t169 * t699 * t2929;
    let t10906 = t169 * t1102 * t1143;
    (t10872, t10878, t10881, t10886, t10897, t10906)
}
