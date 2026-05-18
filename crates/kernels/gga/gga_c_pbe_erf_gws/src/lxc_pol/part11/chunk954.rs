//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 954/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk954<F: Float>(t1036: F, t2591: F, t639: F, t108: F, t267: F, t2740: F, t7068: F, t995: F, t1041: F, t2718: F, t1028: F, t2704: F) -> (F, F, F, F, F) {
    let t24784 = t2591 * t1036;
    let t24785 = t639 * t24784;
    let t24835 = t2740 * t108 * t267;
    let t24848 = t7068 * t995;
    let t24980 = t2718 * t1041;
    let t25049 = t2704 * t1028;
    (t24785, t24835, t24848, t24980, t25049)
}
