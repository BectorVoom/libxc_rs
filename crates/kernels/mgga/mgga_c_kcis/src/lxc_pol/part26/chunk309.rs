//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 309/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk309<F: Float>(t1608: F, t1610: F, t286: F, t1597: F, t1599: F, t1603: F) -> (F, F, F) {
    let t1611 = t1608 * t1610;
    let t1612 = t286 * t1611;
    let t1615 = t1597 + t1599 * t1603 / F::cast_from(576.0_f64) - t1599 * t1612 / F::cast_from(192.0_f64);
    (t1611, t1612, t1615)
}
