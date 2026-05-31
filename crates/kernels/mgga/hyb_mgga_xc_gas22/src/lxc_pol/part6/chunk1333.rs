//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1333/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1333<F: Float>(t28985: F, t28996: F, t29010: F, t29023: F, t788: F, t810: F, t10718: F, t787: F, t811: F, t10555: F, t2229: F, t2233: F, t4108: F) -> (F, F, F, F) {
    let t29028 = F::cast_from(1.0_f64) * t788 * (t28985 + t28996 + t29010 + t29023) * t810;
    let t29029 = t10718 * t787;
    let t29031 = F::cast_from(2.0_f64) * t29029 * t811;
    let t29033 = F::cast_from(1.0_f64) * t10555 * t2229;
    let t29034 = t4108 * t2233;
    (t29028, t29031, t29033, t29034)
}
