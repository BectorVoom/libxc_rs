//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 662/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk662<F: Float>(t1435: F, t1594: F, t1597: F, t1610: F, t747: F, t1609: F, t303: F, t337: F, t280: F, t1303: F, t1625: F, t1349: F, t5164: F) -> (F, F, F, F, F, F, F) {
    let t6018 = t1594 * t1435;
    let t6020 = t1597 * t1435;
    let t6022 = t747 * t1610;
    let t6023 = t1609 * t6022;
    let t6025 = t303 * t337;
    let t6026 = t6025 * t280;
    let t6027 = t1303 * t6026;
    let t6028 = t6027 * t1625;
    let t6030 = t1349 * t5164;
    (t6018, t6020, t6022, t6023, t6026, t6028, t6030)
}
