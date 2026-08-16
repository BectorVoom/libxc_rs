//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1265/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1265<F: Float>(t16131: F, t3781: F, t3820: F, t5481: F, t1319: F, t3809: F, t5513: F, t1330: F, t16078: F, t4714: F, t5567: F, t659: F) -> (F, F, F, F, F) {
    let t16132 = t16131 * t3781;
    let t16134 = t3820 * t5481;
    let t16135 = t16134 * t1319;
    let t16137 = t5513 * t3809;
    let t16141 = t1330 * t16078;
    let t16142 = t4714 * t16141;
    let t16144 = t659 * t5567;
    (t16132, t16135, t16137, t16142, t16144)
}
