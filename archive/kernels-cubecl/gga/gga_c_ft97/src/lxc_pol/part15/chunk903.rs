//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 903/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk903<F: Float>(t15706: F, t3020: F, t1593: F, t4466: F, t419: F, t4487: F, t626: F, t4479: F, t4483: F, t408: F, t15712: F, t1771: F, t4463: F) -> (F, F, F, F, F, F, F, F) {
    let t58407 = t3020 * t15706;
    let t58513 = t1593 * t4466;
    let t58708 = t419 * t626 * t4487;
    let t58719 = t419 * t626 * t4479;
    let t58730 = t419 * t626 * t4483;
    let t58877 = t408 * t4466;
    let t58911 = t3020 * t15712;
    let t58969 = t1771 * t4463;
    (t58407, t58513, t58708, t58719, t58730, t58877, t58911, t58969)
}
