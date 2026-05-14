//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1021/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1021<F: Float>(t20521: F, t4261: F, t9074: F, t1365: F, t20358: F, t6525: F, t19532: F, t20370: F, t2300: F, t23983: F, t6455: F, t1358: F, t9208: F, t20692: F, t1349: F, t9083: F) -> (F, F, F, F, F, F, F) {
    let t30126 = 0.47425011059460249332e-2 * t9074 * t4261 * t20521;
    let t30129 = 0.23712505529730124666e-2 * t6525 * t1365 * t20358;
    let t30132 = 0.142275033178380748e-1 * t9074 * t19532 * t20370;
    let t30135 = 0.47425011059460249332e-2 * t23983 * t2300 * t6455;
    let t30145 = 0.12646669615856066488e-1 * t1358 * t9208;
    let t30148 = 0.47425011059460249332e-2 * t6525 * t1365 * t20692;
    let t30152 = 0.63233348079280332442e-2 * t1349 * t9083;
    (t30126, t30129, t30132, t30135, t30145, t30148, t30152)
}
