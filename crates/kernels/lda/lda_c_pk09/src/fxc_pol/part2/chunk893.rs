//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 893/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk893<F: Float>(t10968: F, t11004: F, t1191: F, t1710: F, t424: F, t2716: F, t6403: F, t2715: F, t4910: F, t6360: F, t1701: F, t2146: F, t6413: F, t2: F, t420: F, t9727: F) -> (F, F, F, F, F, F) {
    let t11007 = t11004 * t1191 * t424 + t10968 * t1710;
    let t11013 = 1.28 * t6403 * t2716;
    let t11014 = t2715 * t4910;
    let t11016 = 1.28 * t6360 * t11014;
    let t11019 = t1701 * t2146;
    let t11020 = t11019 * t6413;
    let t11023 = t420 * t2;
    let t11024 = t11023 * t9727;
    (t11007, t11013, t11016, t11020, t11023, t11024)
}
