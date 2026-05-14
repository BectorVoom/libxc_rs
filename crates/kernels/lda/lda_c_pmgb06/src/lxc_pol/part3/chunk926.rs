//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 926/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk926<F: Float>(t2933: F, t5068: F, t852: F, t2924: F, t5138: F, t2992: F, t5090: F, t1586: F, t764: F, t529: F, t6559: F, t337: F, t5069: F, t5139: F, t11904: F, t5072: F) -> (F, F, F, F, F, F, F) {
    let t12473 = 2.0 / 15.0 * t5068 * t852 * t2933;
    let t12476 = t5138 * t852 * t2924 / 9.0;
    let t12479 = 2.0 / 9.0 * t5138 * t5090 * t2992;
    let t12480 = t764 * t1586;
    let t12484 = 2.0 / 15.0 * t5068 * t6559 * t12480 * t529;
    let t12485 = t12480 * t337;
    let t12488 = 2.0 / 15.0 * t5068 * t5069 * t12485;
    let t12491 = t5138 * t5139 * t12485 / 9.0;
    let t12493 = 4.0 / 15.0 * t11904 * t5072;
    (t12473, t12476, t12479, t12484, t12488, t12491, t12493)
}
