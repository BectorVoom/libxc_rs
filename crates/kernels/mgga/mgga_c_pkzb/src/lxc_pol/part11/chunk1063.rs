//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1063/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1063<F: Float>(t19377: F, t23: F, t23796: F, t2504: F, t28696: F, t28700: F, t28704: F, t28707: F, t28710: F, t28714: F, t28718: F, t3324: F, t6679: F, t8646: F, t8650: F, t8654: F, t8658: F, t980: F) -> (F,) {
    let t28817 = -440.0 / 9.0 * t3324 * t2504 + 80.0 / 27.0 * t980 * t8646 + 160.0 / 9.0 * t23796 * t8650 - 80.0 / 9.0 * t980 * t8654 - 40.0 / 3.0 * t980 * t8658 + 40.0 / 81.0 * t23 * t28696 + 10.0 / 9.0 * t19377 * t28700 - 10.0 / 9.0 * t19377 * t28704 - 10.0 / 3.0 * t6679 * t28707 + 10.0 / 3.0 * t23 * t28710 + 10.0 / 9.0 * t23 * t28714 + 5.0 / 3.0 * t23 * t28718;
    (t28817,)
}
