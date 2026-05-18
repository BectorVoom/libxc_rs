//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 728/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk728<F: Float>(t1798: F, t6292: F, t489: F, t6287: F, t497: F, t1831: F, t1800: F, t1827: F, t501: F, t1971: F, t309: F, t1876: F) -> (F, F, F, F, F, F, F, F) {
    let t7488 = F::new(2.2140749178833072) * t1798 * t6292;
    let t7489 = t489 * t6287;
    let t7494 = t497 * t6287;
    let t7500 = t1831 * t6287;
    let t7501 = t7500 * t1800;
    let t7503 = t1827 * t6287;
    let t7504 = t7503 * t1800;
    let t7506 = t501 * t6287;
    let t7513 = t1971 * t309;
    let t7516 = t1876 * t6287;
    (t7488, t7489, t7494, t7501, t7504, t7506, t7513, t7516)
}
