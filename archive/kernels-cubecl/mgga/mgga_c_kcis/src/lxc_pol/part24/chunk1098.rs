//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1098/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1098<F: Float>(t26791: F, t6533: F, t5329: F, t356: F, t6497: F, t303: F, t26772: F, t6487: F, t6276: F, t7704: F, t4947: F, t26695: F, t6272: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28996 = t26791 * t6533;
    let t28997 = t5329 * t28996;
    let t29000 = t356 * t6497;
    let t29001 = t303 * t29000;
    let t29003 = t26772 * t6487;
    let t29004 = t303 * t29003;
    let t29006 = t7704 * t6276;
    let t29007 = t4947 * t29006;
    let t29010 = t26695 * t6272;
    (t28996, t28997, t29000, t29001, t29003, t29004, t29006, t29007, t29010)
}
