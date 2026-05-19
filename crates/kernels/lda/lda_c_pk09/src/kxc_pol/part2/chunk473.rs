//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 473/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk473<F: Float>(t2594: F, t297: F, t1633: F, t1295: F, t1297: F, t2502: F, t2505: F, t1302: F) -> (F, F, F, F) {
    let t2595 = t2594 * t297;
    let t2596 = t2595 * t1633;
    let t2605 = t1295 - F::cast_from(11.879313099038017_f64) * t2502 + t1297 + F::cast_from(11.879313099038017_f64) * t2505;
    let t2606 = t2605 * t1302;
    (t2595, t2596, t2605, t2606)
}
