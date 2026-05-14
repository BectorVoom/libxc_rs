//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 456/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk456<F: Float>(t6161: F, t684: F, t2606: F, t1449: F, t713: F, t729: F, t762: F, t2469: F, t242: F, t766: F, t2568: F, t6116: F, t6133: F, t6113: F, t6122: F, t6126: F, t6130: F, t6138: F, t6142: F, t6146: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6162 = t6161 * t684;
    let t6163 = t2606 * t6162;
    let t6166 = t1449 * t713;
    let t6168 = t729 * t762 * t6166;
    let t6171 = t2469 * t1449;
    let t6172 = t242 * t6171;
    let t6175 = t1449 * t766;
    let t6176 = t2568 * t6175;
    let t6177 = t242 * t6176;
    let t6181 = t6116 / 6.0;
    let t6184 = t6133 / 3.0;
    let t6187 = t6113 / 4.0 + t6181 + t6122 / 6.0 + t6126 - t6130 / 2.0 + t6184 + t6138 / 3.0 + 2.0 * t6142 - t6146;
    (t6162, t6163, t6166, t6168, t6171, t6172, t6175, t6176, t6177, t6181, t6184, t6187)
}
