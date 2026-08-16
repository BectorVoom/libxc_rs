//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 683/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk683<F: Float>(t2894: F, t4943: F, t291: F, t993: F, t4581: F, t736: F, t992: F) -> (F, F, F, F) {
    let t4944 = t2894 * t4943;
    let t4947 = t993 * t291;
    let t4948 = t4947 * t4581;
    let t4951 = t736 * t992;
    (t4944, t4947, t4948, t4951)
}
