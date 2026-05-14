//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 581/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk581<F: Float>(t3202: F, t4801: F, t3200: F, t1133: F, t1646: F, t3211: F) -> (F, F, F) {
    let t4802 = t3202 * t4801;
    let t4803 = t3200 * t4802;
    let t4805 = t1646 * t1133;
    let t4806 = t3211 * t4805;
    (t4802, t4803, t4806)
}
