//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1017/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1017<F: Float>(t29874: F, t9075: F, t1365: F, t20540: F, t23983: F, t484: F, t9087: F, t145: F, t27835: F, t459: F, t1242: F, t27839: F, t27842: F, t4074: F, t4077: F, t18091: F, t27847: F) -> (F, F, F, F, F, F, F) {
    let t29876 = 0.47425011059460249332e-2 * t29874 * t9075;
    let t29879 = 0.47425011059460249332e-2 * t23983 * t1365 * t20540;
    let t29892 = 0.63233348079280332442e-2 * t484 * t9087;
    let t29896 = t27835 * t145 * t459;
    let t29898 = t27839 * t1242;
    let t29901 = t27842 * t4074 * t4077;
    let t29903 = t27847 * t18091;
    (t29876, t29879, t29892, t29896, t29898, t29901, t29903)
}
