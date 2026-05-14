//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 651/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk651<F: Float>(t1468: F, t536: F, t1747: F, t6302: F, t1798: F, t6488: F, t543: F, t1887: F, t337: F, t1782: F, t1672: F, t1778: F, t6501: F, t6505: F, t6522: F, t6319: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7332 = t536 * t1468;
    let t7333 = t7332 * t1747;
    let t7335 = 4.4281498357666145 * t7333 * t6302;
    let t7337 = 1.4760499452555382 * t1798 * t6488;
    let t7339 = t543 * t543;
    let t7340 = 1.0 / t7339;
    let t7345 = t1887 * t337;
    let t7346 = t7345 * t1782;
    let t7353 = t1778 * t1672;
    let t7362 = 6.25 * t6501;
    let t7363 = 6.25 * t6505;
    let t7367 = 8.333333333333334 * t6522;
    let t7371 = 1.2466946262544771 * t6319;
    (t7335, t7337, t7340, t7346, t7353, t7362, t7363, t7367, t7371)
}
