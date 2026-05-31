//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 668/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk668<F: Float>(t2252: F, t652: F, t256: F, t19: F, t1904: F, t644: F, t647: F, t1432: F, t850: F, t1427: F, t2260: F, t1217: F, t858: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5791 = t2252 * t652;
    let t5793 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t5791 * t256;
    let t5794 = t1904 * t19;
    let t5795 = t5794 * t644;
    let t5797 = F::cast_from(0.12155555555555556_f64) * t5795 * t647;
    let t5798 = t850 * t1432;
    let t5799 = t5798 * t256;
    let t5801 = t2260 * t1427;
    let t5806 = t858 * t1217;
    (t5791, t5793, t5794, t5795, t5797, t5798, t5799, t5801, t5806)
}
