//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2360/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2360<F: Float>(t104729: F, t104976: F, t27290: F, t4028: F, t510: F, t5361: F, t5493: F, t652: F, t7408: F, t8107: F, t97856: F, t97858: F, t97862: F, t97865: F, t97869: F, t97871: F, t97874: F, t97878: F, t97880: F, t97887: F, t97889: F, t97892: F, t97893: F, t97897: F) -> F {
    let t105073 = -F::cast_from(2.0_f64) * t5493 * t652 * t7408 - F::cast_from(2.0_f64) * t104729 * t510 - t104976 * t510 - F::cast_from(4.0_f64) * t27290 * t4028 + F::cast_from(2.0_f64) * t5361 * t8107 - t97856 - t97858 - t97862 - t97865 - t97869 - t97871 + t97874 - t97878 + t97880 + t97887 - t97889 + t97892 - t97893 + t97897;
    t105073
}
