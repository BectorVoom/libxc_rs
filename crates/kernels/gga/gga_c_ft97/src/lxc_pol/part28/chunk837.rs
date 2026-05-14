//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 837/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk837<F: Float>(t5551: F, t79931: F, t373: F, t5555: F, t173: F, t32266: F, t32267: F, t32268: F, t1669: F, t6: F, t92920: F, t32300: F, t66: F, t1616: F, t37481: F, t53: F, t7189: F) -> (F, F, F, F, F, F) {
    let t136693 = t79931 * t5551;
    let t136694 = t5555 * t373;
    let t136714 = t32266 * t32267 * t173 * t32268;
    let t136720 = t1669 * t92920 * t6;
    let t136735 = t32300 * t66;
    let t136736 = t136735 * t1616;
    let t136740 = t37481 * t53 * t7189;
    (t136693, t136694, t136714, t136720, t136736, t136740)
}
