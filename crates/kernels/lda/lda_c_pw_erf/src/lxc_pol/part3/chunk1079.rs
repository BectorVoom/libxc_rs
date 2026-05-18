//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1079/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1079<F: Float>(t12633: F, t2188: F, t3709: F, t2171: F, t3784: F, t3788: F, t4738: F, t5068: F, t518: F, t2168: F, t10508: F, t826: F) -> (F, F, F, F, F, F, F) {
    let t12634 = F::new(8.0) / F::new(9.0) * t12633;
    let t12636 = F::new(4.0) / F::new(5.0) * t3709 * t2188;
    let t12637 = t2171 * t3784;
    let t12638 = F::new(8.0) / F::new(135.0) * t12637;
    let t12639 = t4738 * t3788;
    let t12640 = F::new(16.0) / F::new(15.0) * t12639;
    let t12641 = t5068 * t518;
    let t12643 = F::new(8.0) / F::new(5.0) * t12641 * t2168;
    let t12645 = F::new(8.0) / F::new(15.0) * t10508 * t826;
    (t12634, t12636, t12638, t12640, t12641, t12643, t12645)
}
