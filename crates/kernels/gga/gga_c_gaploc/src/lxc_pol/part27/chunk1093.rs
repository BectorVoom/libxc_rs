//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1093/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1093<F: Float>(t24303: F, t977: F, t10805: F, t5559: F, t841: F, t1052: F, t22139: F, t23575: F, t2972: F, t5552: F, t1960: F, t2728: F, t3073: F, t7822: F, t7332: F, t8862: F) -> (F, F, F, F, F, F, F, F) {
    let t32716 = t24303 * t977;
    let t32719 = 12.0 * t5559 * t10805 * t841;
    let t32720 = t22139 * t1052;
    let t32723 = 4.0 * t23575 * t2972;
    let t32731 = 4.0 * t5552 * t10805;
    let t32734 = 4.0 * t1960 * t3073 * t2728;
    let t32736 = 2.0 * t7822 * t3073;
    let t32740 = 2.0 * t8862 * t7332;
    (t32716, t32719, t32720, t32723, t32731, t32734, t32736, t32740)
}
