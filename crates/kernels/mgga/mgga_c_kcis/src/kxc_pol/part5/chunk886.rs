//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 886/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk886<F: Float>(t3030: F, t961: F, t273: F, t3033: F, t2985: F, t926: F, t257: F, t2984: F, t244: F, t2323: F, t923: F) -> (F, F, F, F, F) {
    let t9630 = 1.0 / t3030 / t961;
    let t9634 = 1.0 / t3033 / t273;
    let t9655 = t926 * t2985;
    let t9659 = 1.0 / t2984 / t257;
    let t9660 = t244 * t9659;
    let t9691 = t2323 * t923;
    (t9630, t9634, t9655, t9660, t9691)
}
