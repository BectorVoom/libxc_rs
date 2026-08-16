//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 953/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk953<F: Float>(t1022: F, t5002: F, t7514: F, t995: F, t1009: F, t2620: F, t587: F, t1014: F, t2718: F, t1001: F, t2704: F, t1006: F, t5357: F) -> (F, F, F, F, F, F) {
    let t23207 = t1022 * t5002;
    let t23336 = t7514 * t995;
    let t23816 = t2620 * t1009;
    let t23817 = t587 * t23816;
    let t24074 = t2718 * t1014;
    let t24088 = t2704 * t1001;
    let t24131 = t1006 * t5357;
    (t23207, t23336, t23817, t24074, t24088, t24131)
}
