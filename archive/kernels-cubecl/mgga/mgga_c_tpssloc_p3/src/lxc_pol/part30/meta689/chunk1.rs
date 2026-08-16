//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2196/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2196<F: Float>(t1442: F, t1869: F, t19289: F, t25958: F, t33085: F, t4073: F, t6287: F, t6515: F, t672: F, t96686: F, t97862: F, t97865: F, t97869: F, t97871: F, t97874: F, t97878: F, t97880: F, t97887: F, t97889: F, t97892: F, t97893: F, t97897: F, t97899: F, t97905: F) -> F {
    let t97906 = -F::cast_from(2.0_f64) * t1442 * t25958 - t1869 * t19289 - F::cast_from(4.0_f64) * t33085 * t4073 - t6287 * t6515 - F::cast_from(2.0_f64) * t672 * t96686 - t97862 - t97865 - t97869 - t97871 + t97874 - t97878 + t97880 + t97887 - t97889 + t97892 - t97893 + t97897 + t97899 - t97905;
    t97906
}
