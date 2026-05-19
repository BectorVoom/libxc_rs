//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 538/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk538<F: Float>(t2747: F, t318: F, t473: F, t174: F, t335: F, t936: F, t998: F, t155: F, t912: F, t914: F, t1035: F, t344: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2748 = F::cast_from(0.8591714644109227_f64) * t2747;
    let t2749 = t473 * t318;
    let t2751 = t174 * t2749 * t335;
    let t2752 = F::cast_from(0.07123333333333333_f64) * t2751;
    let t2754 = t174 * t998 * t936;
    let t2755 = F::new(0.053425) * t2754;
    let t2758 = t174 * t155 * t912 * t914;
    let t2759 = F::new(0.10685) * t2758;
    let t2760 = t344 * t1035;
    (t2748, t2749, t2751, t2752, t2754, t2755, t2758, t2759, t2760)
}
