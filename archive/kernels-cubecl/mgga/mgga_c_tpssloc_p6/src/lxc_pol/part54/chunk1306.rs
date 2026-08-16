//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1306/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1306<F: Float>(t112797: F, t32844: F, t13242: F, t232: F, t30714: F, t4180: F, t234: F, t240: F, t241: F, t4248: F, t776: F, t812: F, t9646: F) -> (F, F, F) {
    let t118535 = t112797 * t32844;
    let t118539 = t30714 * t4180 * t13242 * t232;
    let t118546 = t812 * t234 * t240 * t241 * t9646 * t4248 * t776;
    (t118535, t118539, t118546)
}
