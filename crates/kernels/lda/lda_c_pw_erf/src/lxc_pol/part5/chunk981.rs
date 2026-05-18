//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 981/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk981<F: Float>(t133: F, t14584: F, t21: F, t411: F, t635: F, t1652: F, t763: F, t4: F, t474: F, t5607: F, t2: F, t39: F, t756: F) -> (F, F, F, F, F, F) {
    let t14585 = t133 * t14584;
    let t14639 = t21 * t635 * t411;
    let t14640 = t1652 * t763 * t14639;
    let t14641 = F::new(1.9486833333333333) * t14640;
    let t14650 = t4 * t474 * t411;
    let t14651 = t5607 * t14650;
    let t14652 = F::new(3.8973666666666666) * t14651;
    let t14654 = t756 * t2 * t39;
    (t14585, t14639, t14641, t14650, t14652, t14654)
}
