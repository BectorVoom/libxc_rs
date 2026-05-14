//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1140/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1140<F: Float>(t1150: F, t51200: F, t14028: F, t3295: F, t4023: F, t9172: F, t54107: F, t54109: F, t54111: F, t54114: F, t54115: F, t54118: F, t54120: F, t54122: F, t54124: F, t14101: F, t8910: F) -> (F, F) {
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54129 = 7.0 / 576.0 * t54128;
    let t54130 = t9172 * t4023;
    let t54132 = t54107 / 96.0 - t54109 / 48.0 + t54111 / 192.0 + t54114 - t54115 / 192.0 + t54118 + t54120 / 48.0 - t54122 / 48.0 + t54124 / 192.0 + 119.0 / 3456.0 * t54126 - t54129 + t54130 / 96.0;
    let t54133 = t14101 * t8910;
    (t54132, t54133)
}
