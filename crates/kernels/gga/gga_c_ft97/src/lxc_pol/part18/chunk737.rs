//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 737/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk737<F: Float>(t12645: F, t167: F, t2185: F, t3408: F, t609: F, t574: F, t605: F, t3450: F, t616: F, t2142: F, t3455: F, t12610: F, t12614: F, t12617: F, t12620: F, t12622: F, t12626: F, t12630: F, t12634: F, t12638: F, t12642: F, t12644: F, t1901: F, t446: F) -> (F, F) {
    let t12647 = t2185 * t167 * t12645;
    let t12650 = t3408 * t609;
    let t12652 = t574 * t605 * t12650;
    let t12656 = t2185 * t616 * t3450;
    let t12660 = t574 * t2142 * t3455;
    let t12663 = -2.0 / 9.0 * t1901 * t12610 + 2.0 / 3.0 * t446 * t12614 - 4.0 / 81.0 * t12617 + t12620 - 2.0 / 9.0 * t446 * t12622 - 2.0 / 9.0 * t1901 * t12626 - 2.0 / 3.0 * t446 * t12630 - 2.0 / 3.0 * t446 * t12634 - t446 * t12638 / 3.0 - t12642 - t12644 + 4.0 / 3.0 * t446 * t12647 + 2.0 / 3.0 * t446 * t12652 + 4.0 / 3.0 * t446 * t12656 + 2.0 / 3.0 * t446 * t12660;
    (t12650, t12663)
}
