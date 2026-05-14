//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1081/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1081<F: Float>(t19778: F, t19805: F, t16638: F, t10535: F, t496: F, t501: F, t16626: F, t16631: F, t16701: F, t16873: F, t19757: F, t19759: F, t19776: F, t19798: F, t19804: F, t19823: F, t28970: F) -> (F, F, F, F, F, F) {
    let t29118 = 3.0 * t19778;
    let t29119 = 360.0 * t19805;
    let t29120 = 60.0 * t16638;
    let t29121 = t496 * t10535;
    let t29122 = 4.0 * t29121;
    let t29123 = t501 * t10535;
    let t29124 = 4.0 * t29123;
    let t29125 = t28970 + t19757 + t19759 + t19776 + t29118 + t16626 - t16631 + t19798 - t19804 - t29119 + t29120 + t16873 + t29122 - t29124 + t16701 - t19823;
    (t29118, t29119, t29120, t29122, t29124, t29125)
}
