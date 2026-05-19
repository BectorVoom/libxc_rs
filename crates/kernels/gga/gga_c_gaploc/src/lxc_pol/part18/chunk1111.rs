//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1111/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1111<F: Float>(t2684: F, t7354: F, t9829: F, t1391: F, t9833: F, t15490: F, t7584: F, t9438: F, t21456: F, t2365: F, t7390: F, t7416: F, t9834: F) -> (F, F, F, F, F) {
    let t28987 = t2684 * t7354 * t9829;
    let t28988 = F::cast_from(0.1022478025437886658e1_f64) * t28987;
    let t28990 = t2684 * t1391 * t9833;
    let t28991 = F::cast_from(0.5396411800922179584e0_f64) * t28990;
    let t29001 = t7584 * t9438 * t15490;
    let t29009 = F::cast_from(0.59584149919750711116e-1_f64) * t7390 * t2365 * t21456;
    let t29011 = F::cast_from(0.17041300423964777634e0_f64) * t7416 * t9834;
    (t28988, t28991, t29001, t29009, t29011)
}
