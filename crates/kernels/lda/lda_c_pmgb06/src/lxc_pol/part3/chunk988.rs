//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 988/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk988<F: Float>(t4844: F, t486: F, t9089: F, t9091: F, t9093: F, t5105: F, t161: F, t489: F, t4953: F, t132: F, t137: F, t2106: F, t3441: F) -> (F, F, F, F, F, F, F) {
    let t11757 = t486 * t4844;
    let t11758 = t11757 / F::new(45.0);
    let t11759 = t9089 / F::new(15.0);
    let t11760 = t9091 / F::new(45.0);
    let t11761 = t9093 / F::new(15.0);
    let t11762 = t486 * t5105;
    let t11763 = F::new(2.0) / F::new(15.0) * t11762;
    let t11765 = t161 * t489 * t4953;
    let t11766 = t11765 / F::new(15.0);
    let t11770 = t132 * t137 * t2106 * t3441 / F::new(30.0);
    (t11758, t11759, t11760, t11761, t11763, t11766, t11770)
}
