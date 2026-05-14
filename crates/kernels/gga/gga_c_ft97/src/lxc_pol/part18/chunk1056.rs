//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1056/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1056<F: Float>(t378: F, t7368: F, t137: F, t8906: F, t6: F, t8908: F, t133: F, t542: F, t7334: F, t527: F, t8832: F, t1995: F, t8851: F, t23: F, t32905: F, t153: F, t1984: F, t22: F, t36452: F, t37991: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t39749 = t378 * t7368;
    let t39801 = 1.0 / t8906 / t137;
    let t39846 = t8908 * t6;
    let t39847 = t133 * t39846;
    let t39852 = t542 * t7334;
    let t40081 = t527 * t8832;
    let t40087 = t1995 * t8851;
    let t40223 = t1995 * t8832;
    let t40227 = t527 * t8851;
    let t40234 = t542 * t39846;
    let t40266 = t23 * t32905;
    let t40280 = 1.0 / t153 / t37991 / t22 / t1984 / t36452 / 96.0;
    (t39749, t39801, t39847, t39852, t40081, t40087, t40223, t40227, t40234, t40266, t40280)
}
