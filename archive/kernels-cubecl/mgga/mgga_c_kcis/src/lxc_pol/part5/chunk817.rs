//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 817/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk817<F: Float>(t1022: F, t6486: F, t3227: F, t1092: F, t1767: F, t1773: F) -> (F, F, F, F) {
    let t6487 = t1022 * t6486;
    let t6488 = t3227 * t6487;
    let t6489 = t1092 * t6488;
    let t6491 = t1767 * t1773;
    (t6487, t6488, t6489, t6491)
}
