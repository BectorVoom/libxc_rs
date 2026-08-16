//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 954/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk954<F: Float>(t1561: F, t2487: F, t2559: F, t6022: F, t1435: F, t2555: F, t2552: F, t131: F, t2143: F, t309: F, t319: F, t2606: F, t5747: F) -> (F, F, F, F, F, F, F) {
    let t10044 = t1561 * t2487;
    let t10048 = t2559 * t6022;
    let t10050 = t2555 * t1435;
    let t10052 = t2552 * t1435;
    let t10059 = t309 * t131 * t2143;
    let t10060 = t319 * t10059;
    let t10062 = t2606 * t5747;
    (t10044, t10048, t10050, t10052, t10059, t10060, t10062)
}
