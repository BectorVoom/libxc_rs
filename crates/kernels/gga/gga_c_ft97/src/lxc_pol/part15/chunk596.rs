//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 596/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk596<F: Float>(t15630: F, t534: F, t408: F, t4491: F, t1710: F, t4474: F, t8051: F, t4455: F, t458: F, t4417: F, t7763: F, t7800: F, t4459: F, t4463: F, t4466: F, t77: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15680 = t534 * t15630;
    let t15706 = t408 * t4491;
    let t15712 = t1710 * t4474;
    let t15716 = t8051 * t4474;
    let t15734 = t458 * t4455;
    let t15736 = t7763 * t4417;
    let t15741 = t7800 * t4417;
    let t15750 = t458 * t4459;
    let t15760 = t458 * t4463;
    let t15781 = t77 * t4466;
    (t15680, t15706, t15712, t15716, t15734, t15736, t15741, t15750, t15760, t15781)
}
