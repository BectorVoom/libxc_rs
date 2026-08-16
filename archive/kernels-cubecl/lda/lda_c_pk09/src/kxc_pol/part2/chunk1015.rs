//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1015/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1015<F: Float>(t10968: F, t11004: F, t1191: F, t1710: F, t424: F, t2716: F, t6403: F, t2715: F, t4910: F, t6360: F, t1701: F, t2146: F) -> (F, F, F, F) {
    let t11007 = t11004 * t1191 * t424 + t10968 * t1710;
    let t11013 = F::cast_from(1.28_f64) * t6403 * t2716;
    let t11014 = t2715 * t4910;
    let t11016 = F::cast_from(1.28_f64) * t6360 * t11014;
    let t11019 = t1701 * t2146;
    (t11007, t11013, t11016, t11019)
}
