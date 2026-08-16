//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2602/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2602<F: Float>(t11702: F, t5002: F, t11708: F, t15502: F, t15506: F, t13969: F, t15554: F, t3506: F, t10469: F, t1720: F, t10471: F, t11737: F) -> (F, F, F, F, F, F, F) {
    let t52801 = t5002 * t11702;
    let t52810 = t11708 * t15502;
    let t52813 = t11708 * t15506;
    let t52817 = t3506 * t13969 * t15554;
    let t52834 = t1720 * t10469;
    let t52835 = t52834 * t10471;
    let t52836 = t52835 * t11737;
    (t52801, t52810, t52813, t52817, t52834, t52835, t52836)
}
