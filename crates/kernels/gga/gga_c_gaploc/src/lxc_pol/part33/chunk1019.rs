//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1019/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1019<F: Float>(t29874: F, t9205: F, t123: F, t21004: F, t2326: F, t9074: F, t6466: F, t9204: F, t20572: F, t2325: F, t882: F, t883: F, t2312: F, t9090: F, t2321: F, t6776: F) -> (F, F, F, F, F, F) {
    let t30005 = 0.142275033178380748e-1 * t29874 * t9205;
    let t30009 = 0.142275033178380748e-1 * t9074 * t21004 * t123 * t2326;
    let t30014 = 0.71137516589190373998e-2 * t9074 * t9204 * t6466;
    let t30049 = 0.23712505529730124666e-2 * t882 * t2325 * t883 * t20572;
    let t30091 = 0.47425011059460249332e-2 * t2312 * t9090;
    let t30094 = 0.23712505529730124666e-2 * t882 * t6776 * t2321;
    (t30005, t30009, t30014, t30049, t30091, t30094)
}
