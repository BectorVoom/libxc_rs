//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 727/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk727<F: Float>(t545: F, t6287: F, t1744: F, t337: F, t429: F, t1741: F, t2042: F, t1468: F, t544: F, t1747: F, t6302: F, t1838: F, t7273: F) -> (F, F, F, F, F) {
    let t7469 = t545 * t6287;
    let t7472 = t1744 * t337;
    let t7473 = t7472 * t429;
    let t7474 = t1741 * t7473;
    let t7475 = t7474 * t2042;
    let t7481 = t544 * t1468;
    let t7482 = t7481 * t1747;
    let t7484 = F::new(4.855032390388656) * t7482 * t6302;
    let t7485 = t7273 * t1838;
    (t7469, t7473, t7475, t7484, t7485)
}
