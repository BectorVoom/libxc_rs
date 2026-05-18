//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1047/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1047<F: Float>(t12456: F, t432: F, t4817: F, t835: F, t9266: F, t1977: F, t3223: F, t11862: F, t160: F, t1983: F, t2983: F, t5068: F, t5090: F) -> (F, F, F, F, F, F) {
    let t12457 = F::new(4.0) / F::new(405.0) * t12456;
    let t12459 = t432 * t4817 / F::new(5.0);
    let t12460 = t9266 * t835;
    let t12461 = F::new(2.0) / F::new(135.0) * t12460;
    let t12462 = t3223 * t1977;
    let t12463 = F::new(2.0) / F::new(135.0) * t12462;
    let t12465 = t160 * t11862 * t1983;
    let t12466 = F::new(32.0) / F::new(135.0) * t12465;
    let t12469 = F::new(2.0) / F::new(15.0) * t5068 * t5090 * t2983;
    (t12457, t12459, t12461, t12463, t12466, t12469)
}
