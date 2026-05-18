//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 688/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk688<F: Float>(t1506: F, t7962: F, t2256: F, t4409: F, t251: F, t4414: F, t1598: F) -> (F, F, F, F) {
    let t7963 = t1506 * t7962;
    let t7964 = t4409 * t2256;
    let t7967 = t4414 * t251;
    let t7968 = t7967 * t1598;
    (t7963, t7964, t7967, t7968)
}
