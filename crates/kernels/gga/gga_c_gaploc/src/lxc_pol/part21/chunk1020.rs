//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1020/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1020<F: Float>(t3122: F, t6338: F, t1349: F, t9070: F, t20237: F, t2321: F, t9074: F, t23927: F, t4255: F, t883: F, t9204: F, t123: F, t20008: F, t6486: F, t1358: F, t9205: F) -> (F, F, F, F, F, F, F, F) {
    let t30096 = 0.23712505529730124666e-2 * t6338 * t3122;
    let t30098 = 0.63233348079280332442e-2 * t1349 * t9070;
    let t30103 = 0.23712505529730124666e-2 * t9074 * t20237 * t2321;
    let t30105 = 0.47425011059460249332e-2 * t23927 * t9070;
    let t30110 = t883 * t4255;
    let t30113 = 0.16598753870811087267e-1 * t9074 * t9204 * t30110;
    let t30118 = 0.284550066356761496e-1 * t9074 * t20008 * t123 * t6486;
    let t30120 = 0.18970004423784099732e-1 * t1358 * t9205;
    (t30096, t30098, t30103, t30105, t30110, t30113, t30118, t30120)
}
