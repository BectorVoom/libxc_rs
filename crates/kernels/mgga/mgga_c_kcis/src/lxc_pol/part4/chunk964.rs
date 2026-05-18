//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 964/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk964<F: Float>(t25: F, t3041: F, t285: F, t3030: F, t961: F, t273: F, t3033: F, t2930: F, t930: F, t2985: F, t926: F, t257: F, t2984: F) -> (F, F, F, F, F, F) {
    let t9622 = t25 * t3041;
    let t9623 = t285 * t9622;
    let t9630 = F::new(1.0) / t3030 / t961;
    let t9634 = F::new(1.0) / t3033 / t273;
    let t9650 = t2930 * t930;
    let t9655 = t926 * t2985;
    let t9659 = F::new(1.0) / t2984 / t257;
    (t9623, t9630, t9634, t9650, t9655, t9659)
}
