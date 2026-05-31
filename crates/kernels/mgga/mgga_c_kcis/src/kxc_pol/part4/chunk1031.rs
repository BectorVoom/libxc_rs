//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1031/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1031<F: Float>(t12838: F, t612: F, t110: F, t1611: F, t1599: F, t1607: F, t3970: F, t4441: F, t4439: F, t4425: F, t4446: F, t1590: F) -> (F, F, F, F, F, F) {
    let t12840 = F::cast_from(5.0_f64) / F::cast_from(2592.0_f64) * t612 * t12838;
    let t12841 = t110 * t1611;
    let t12842 = t1599 * t12841;
    let t12844 = t3970 * t1607;
    let t12845 = t12844 * t4441;
    let t12846 = t4439 * t12845;
    let t12848 = t4425 * t4446;
    let t12849 = t1599 * t12848;
    let t12856 = t1590 * t1590;
    (t12840, t12842, t12844, t12846, t12849, t12856)
}
