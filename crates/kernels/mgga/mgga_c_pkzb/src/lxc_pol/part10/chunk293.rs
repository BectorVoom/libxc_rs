//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 293/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk293<F: Float>(t179: F, t824: F, t932: F, t385: F, t404: F, t906: F, t909: F, t918: F, t923: F, t929: F) -> (F, F) {
    let t934 = t179 * t932 * t824;
    let t937 = t906 - t385 * t909 / 96.0 + 0.21437009059034868486e-3 * t918 * t923 + t929 - 0.42874018118069736972e-3 * t404 * t934;
    (t934, t937)
}
