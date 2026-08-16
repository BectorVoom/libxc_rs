//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1372/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1372<F: Float>(t21110: F, t5976: F, t21073: F, t21078: F, t5968: F, t1419: F, t1961: F, t16416: F, t16388: F, t5463: F, t1317: F, t2018: F) -> (F, F, F, F, F, F) {
    let t22604 = t5976 * t21110;
    let t22607 = t5976 * t21073;
    let t22610 = t5968 * t21078;
    let t22615 = t1961 * t1419;
    let t22616 = t16416 * t22615;
    let t22619 = t5463 * t16388;
    let t22623 = t2018 * t1317;
    (t22604, t22607, t22610, t22616, t22619, t22623)
}
