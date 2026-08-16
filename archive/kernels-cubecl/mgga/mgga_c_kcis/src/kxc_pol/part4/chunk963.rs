//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 963/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk963<F: Float>(t9588: F, t3206: F, t9429: F, t2867: F, t987: F, t25: F, t2912: F, t285: F, t1004: F, t110: F, t2884: F, t984: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t9589 = t9588 * sigma0;
    let t9600 = t9429 * t3206;
    let t9608 = t2867 * t987;
    let t9610 = t25 * t2912;
    let t9611 = t285 * t9610;
    let t9613 = t110 * t1004;
    let t9614 = t285 * t9613;
    let t9620 = t984 * t2884;
    (t9589, t9600, t9608, t9611, t9614, t9620)
}
