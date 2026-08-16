//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 916/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk916<F: Float>(t17837: F, t4952: F, t13519: F, t5019: F, t17831: F, t3771: F, t9523: F, t1611: F, t236: F, t806: F, t5045: F, t626: F, t701: F) -> (F, F, F, F, F) {
    let t65695 = t17837 * t4952;
    let t65702 = t13519 * t5019;
    let t65735 = t3771 * t17831 * t9523;
    let t65743 = t3771 * t236 * t1611 * t806;
    let t65850 = t701 * t626 * t5045;
    (t65695, t65702, t65735, t65743, t65850)
}
