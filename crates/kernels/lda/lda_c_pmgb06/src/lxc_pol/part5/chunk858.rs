//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 858/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk858<F: Float>(t14549: F, t5891: F, t707: F, t5895: F, t1770: F, t419: F, t4238: F, t794: F, t2257: F, t4042: F, t301: F, t413: F, t5575: F, t1183: F, t2174: F, t123: F, t2822: F, t868: F) -> (F, F, F, F, F, F, F, F) {
    let t14550 = 0.1890324433388467 * t14549;
    let t14569 = t707 * t5891;
    let t14570 = 0.11974234010254609 * t14569;
    let t14571 = t707 * t5895;
    let t14575 = t4238 * t794 * t419 * t1770;
    let t14601 = t2257 * t4042;
    let t14639 = t5575 * t413 * t301;
    let t14640 = 0.0017434044910732151 * t14639;
    let t14642 = t2174 * t1183 * t301;
    let t14666 = t123 * t2822 * t868;
    (t14550, t14570, t14571, t14575, t14601, t14640, t14642, t14666)
}
