//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 717/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk717<F: Float>(t10542: F, t10554: F, t10559: F, t10563: F, t10566: F, t10602: F, t10604: F, t10692: F, t10700: F, t12065: F, t12117: F, t1987: F, t240: F, t4764: F, t5423: F, t4753: F) -> (F, F) {
    let t12128 = 0.35089340384731224426e1 * t5423 * t4764 + t10542 + t240 * (t12065 + t12117) - 0.35089340384731224426e1 * t1987 * t10554 + 0.35089340384731224426e1 * t1987 * t10604 - t10559 + t10563 - t10566 - t10602 - 0.1025389702100779493e4 * t1987 * t10700 + 0.1038945353962551798e3 * t1987 * t10692;
    let t12131 = t240 * t4753;
    (t12128, t12131)
}
