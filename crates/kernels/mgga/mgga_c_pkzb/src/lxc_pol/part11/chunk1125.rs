//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1125/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1125<F: Float>(t1769: F, t8988: F, t3457: F, t5296: F, t3401: F, t568: F, t1020: F, t2575: F, t20010: F, t8955: F, t5257: F, t8901: F) -> (F, F, F, F, F, F) {
    let t24169 = t1769 * t8988;
    let t24171 = t5296 * t3457;
    let t24189 = t3401 * t568;
    let t24194 = t1020 * t2575;
    let t24215 = t20010 * t8955;
    let t24217 = t5257 * t8901;
    (t24169, t24171, t24189, t24194, t24215, t24217)
}
