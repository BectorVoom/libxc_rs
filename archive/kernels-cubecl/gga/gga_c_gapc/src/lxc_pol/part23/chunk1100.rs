//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1100/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1100<F: Float>(t11794: F, t7927: F, t9554: F, t126: F, t671: F, t128: F, t314: F, t786: F, t3327: F, t7451: F, t15507: F, t22: F, t5: F) -> (F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t33653 = t11794 * t7927 * t9554;
    let t33655 = t126 * t671;
    let t33657 = t314 * t128;
    let t33658 = t33657 * t786;
    let t33660 = t7451 * t33655 * t3327 * t33658;
    let t33666 = F::cast_from(1.0_f64) / t22 / t15507 * pi * t5;
    (t33653, t33655, t33658, t33660, t33666)
}
