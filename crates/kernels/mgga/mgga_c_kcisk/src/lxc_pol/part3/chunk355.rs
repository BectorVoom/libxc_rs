//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 355/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk355<F: Float>(t1636: F, t1801: F, t1800: F, t1799: F, t167: F, t568: F) -> (F, F, F, F) {
    let t1802 = t1801 * t1636;
    let t1803 = t1800 * t1802;
    let t1804 = t1799 * t1803;
    let t1806 = t167 * t568;
    (t1802, t1803, t1804, t1806)
}
