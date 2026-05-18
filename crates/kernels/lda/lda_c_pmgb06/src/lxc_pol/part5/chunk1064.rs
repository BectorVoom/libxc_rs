//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1064/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1064<F: Float>(t2002: F, t6365: F, t6275: F, t6372: F, t16213: F, t16215: F, t16217: F, t16219: F, t12252: F, t132: F, t137: F, t2604: F) -> (F, F, F, F, F, F, F) {
    let t19736 = F::new(2.0) / F::new(15.0) * t2002 * t6365;
    let t19738 = F::new(4.0) / F::new(15.0) * t6275 * t6372;
    let t19739 = F::new(4.0) / F::new(45.0) * t16213;
    let t19740 = F::new(8.0) / F::new(45.0) * t16215;
    let t19741 = F::new(4.0) / F::new(27.0) * t16217;
    let t19742 = F::new(8.0) / F::new(27.0) * t16219;
    let t19746 = t132 * t137 * t12252 * t2604 / F::new(5.0);
    (t19736, t19738, t19739, t19740, t19741, t19742, t19746)
}
