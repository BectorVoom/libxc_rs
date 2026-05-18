//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1091/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1091<F: Float>(t16524: F, t1423: F, t7646: F, t2485: F, t5220: F, t7574: F, t2481: F, t132: F, t435: F, t7502: F, t16535: F, t16537: F) -> (F, F, F, F, F, F, F, F) {
    let t20127 = F::new(8.0) / F::new(27.0) * t16524;
    let t20128 = t1423 * t7646;
    let t20129 = F::new(4.0) / F::new(27.0) * t20128;
    let t20130 = t5220 * t2485;
    let t20131 = F::new(2.0) / F::new(27.0) * t20130;
    let t20132 = t1423 * t7574;
    let t20133 = F::new(2.0) / F::new(45.0) * t20132;
    let t20134 = t5220 * t2481;
    let t20135 = F::new(2.0) / F::new(45.0) * t20134;
    let t20137 = t132 * t435 * t7502;
    let t20138 = t20137 / F::new(45.0);
    let t20139 = F::new(2.0) / F::new(15.0) * t16535;
    let t20140 = F::new(2.0) / F::new(15.0) * t16537;
    (t20127, t20129, t20131, t20133, t20135, t20138, t20139, t20140)
}
