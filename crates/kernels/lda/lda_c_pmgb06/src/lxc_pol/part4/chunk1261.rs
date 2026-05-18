//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1261/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1261<F: Float>(t12822: F, t4954: F, t831: F, t5432: F, t853: F, t161: F, t489: F, t6460: F, t12825: F, t1848: F, t2101: F, t12828: F) -> (F, F, F, F, F, F, F) {
    let t16577 = F::new(4.0) / F::new(45.0) * t12822;
    let t16579 = t831 * t4954 / F::new(15.0);
    let t16581 = t5432 * t853 / F::new(15.0);
    let t16583 = t161 * t489 * t6460;
    let t16584 = F::new(4.0) / F::new(45.0) * t16583;
    let t16585 = F::new(4.0) / F::new(45.0) * t12825;
    let t16587 = F::new(2.0) / F::new(15.0) * t1848 * t2101;
    let t16588 = F::new(4.0) / F::new(135.0) * t12828;
    (t16577, t16579, t16581, t16584, t16585, t16587, t16588)
}
