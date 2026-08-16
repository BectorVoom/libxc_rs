//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1417/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1417<F: Float>(t208: F, t213: F, t579: F, t6716: F, t588: F, t6717: F, t97: F, t1696: F, t2414: F, t6721: F, t15116: F, t16874: F, t16876: F, t16878: F, t16881: F, t16883: F, t16885: F, t16886: F, t16891: F, t16894: F, t16895: F, t205: F) -> F {
    let t18274 = t6716 * t579 * t208 * t213;
    let t18277 = t6717 * t97 * t588;
    let t18281 = t2414 * t1696 * t208 * t213;
    let t18284 = t6721 * t97 * t588;
    let t18286 = t16874 + t15116 * t205 * t208 * t213 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t18274 + F::cast_from(0.12155555555555556_f64) * t18277 + t18281 / F::cast_from(3.0_f64) + F::cast_from(0.12155555555555556_f64) * t18284 - t16876 - t16878 + t16881 + t16883 - t16885 - t16886 - t16891 + t16894 + t16895;
    t18286
}
