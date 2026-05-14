//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1009/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1009<F: Float>(t2684: F, t7354: F, t9829: F, t1391: F, t9833: F, t15490: F, t7584: F, t9438: F, t21456: F, t2365: F, t7390: F, t7416: F, t9834: F, t2672: F, t6134: F, t7372: F) -> (F, F, F, F, F, F) {
    let t28987 = t2684 * t7354 * t9829;
    let t28988 = 0.1022478025437886658e1 * t28987;
    let t28990 = t2684 * t1391 * t9833;
    let t28991 = 0.5396411800922179584e0 * t28990;
    let t29001 = t7584 * t9438 * t15490;
    let t29009 = 0.59584149919750711116e-1 * t7390 * t2365 * t21456;
    let t29011 = 0.17041300423964777634e0 * t7416 * t9834;
    let t29014 = 0.59584149919750711116e-1 * t6134 * t2672 * t7372;
    (t28988, t28991, t29001, t29009, t29011, t29014)
}
