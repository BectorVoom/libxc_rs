//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 354/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk354<F: Float>(t1817: F, t382: F, t1798: F, t1802: F, t1806: F, t1810: F, t1814: F) -> (F, F) {
    let t1818 = t382 * t1817;
    let t1820 = t1798 / 16.0 - t1802 / 16.0 + t1806 / 24.0 - t1810 / 256.0 + t1814 / 256.0 - t1818 / 192.0;
    (t1818, t1820)
}
