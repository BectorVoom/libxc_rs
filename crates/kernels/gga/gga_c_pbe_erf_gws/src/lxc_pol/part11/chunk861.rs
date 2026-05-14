//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 861/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk861<F: Float>(t23816: F, t587: F, t1014: F, t2718: F, t1001: F, t2704: F, t1006: F, t5357: F, t1036: F, t2591: F, t639: F, t108: F, t267: F, t2740: F, t7068: F, t995: F) -> (F, F, F, F, F, F, F) {
    let t23817 = t587 * t23816;
    let t24074 = t2718 * t1014;
    let t24088 = t2704 * t1001;
    let t24131 = t1006 * t5357;
    let t24784 = t2591 * t1036;
    let t24785 = t639 * t24784;
    let t24835 = t2740 * t108 * t267;
    let t24848 = t7068 * t995;
    (t23817, t24074, t24088, t24131, t24785, t24835, t24848)
}
