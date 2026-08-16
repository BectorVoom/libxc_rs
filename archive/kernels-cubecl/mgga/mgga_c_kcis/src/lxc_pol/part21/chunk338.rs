//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 338/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk338<F: Float>(t1001: F, t1704: F, t286: F, t1700: F, t285: F, t989: F, t991: F) -> (F, F, F) {
    let t1705 = t1001 * t1704;
    let t1706 = t286 * t1705;
    let t1709 = t989 + t991 * t1700 / F::cast_from(288.0_f64) - t285 * t1706 / F::cast_from(96.0_f64);
    (t1705, t1706, t1709)
}
