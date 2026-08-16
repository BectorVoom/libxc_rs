//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 844/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk844<F: Float>(t2142: F, t4733: F, t574: F, t1053: F, t3408: F, t605: F, t1017: F, t3565: F, t1060: F, t3052: F, t569: F, t4462: F, t616: F) -> (F, F, F, F, F) {
    let t17115 = t574 * t2142 * t4733;
    let t17118 = t3408 * t1053;
    let t17120 = t574 * t605 * t17118;
    let t17123 = t1017 * t3565;
    let t17125 = t574 * t605 * t17123;
    let t17129 = t569 * t1060 * t3052;
    let t17133 = t569 * t616 * t4462;
    (t17115, t17120, t17125, t17129, t17133)
}
