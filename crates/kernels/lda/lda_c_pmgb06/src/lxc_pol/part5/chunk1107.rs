//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1107/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1107<F: Float>(t12691: F, t20148: F, t5068: F, t20152: F, t5139: F, t13068: F, t5138: F, t13672: F, t20156: F, t5069: F, t1447: F, t7656: F) -> (F, F, F, F, F) {
    let t20308 = F::new(4.0) / F::new(15.0) * t5068 * t12691 * t20148;
    let t20311 = F::new(2.0) / F::new(5.0) * t5068 * t5139 * t20152;
    let t20314 = F::new(2.0) / F::new(3.0) * t5138 * t13068 * t20152;
    let t20317 = F::new(8.0) / F::new(15.0) * t13672 * t5069 * t20156;
    let t20318 = t1447 * t7656;
    (t20308, t20311, t20314, t20317, t20318)
}
