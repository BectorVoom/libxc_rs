//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 340/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk340<F: Float>(t1040: F, t1664: F, t1671: F, t1036: F, t1670: F, t245: F) -> (F, F) {
    let t1724 = F::cast_from(0.1982e-1_f64) * t1671 - t1040 - F::cast_from(0.41275e-2_f64) * t1664;
    let t1727 = t1036 * t1670 / F::cast_from(4.0_f64) + t245 * t1724 / F::cast_from(2.0_f64);
    (t1724, t1727)
}
