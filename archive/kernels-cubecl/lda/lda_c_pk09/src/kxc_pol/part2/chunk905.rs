//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 905/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk905<F: Float>(t355: F, t9602: F, t1279: F, t2487: F, t359: F, t1303: F, t1310: F, t1287: F, t1283: F, t2501: F, t5042: F) -> (F, F, F, F, F, F, F) {
    let t9603 = t355 * t9602;
    let t9606 = t1279 * t2487;
    let t9609 = t359 * t9602;
    let t9612 = t1303 * t2487;
    let t9615 = t1310 * t9602;
    let t9616 = t9615 * t1287;
    let t9618 = t1283 * t9602;
    let t9619 = t9618 * t1287;
    let t9623 = t5042 * t2501;
    (t9603, t9606, t9609, t9612, t9616, t9619, t9623)
}
