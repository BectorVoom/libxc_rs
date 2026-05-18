//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1189/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1189<F: Float>(t36273: F, t4680: F, t7575: F, t8609: F, t7564: F, t8613: F, t1181: F, t4718: F, t604: F, t7426: F, t31349: F, t3360: F, t4284: F) -> (F, F, F, F, F) {
    let t36274 = F::new(0.10718504529517434243e-2) * t36273;
    let t36276 = t7575 * t4680 * t8609;
    let t36279 = t7564 * t4680 * t8613;
    let t36283 = t7426 * t1181 * t604 * t4718;
    let t36284 = F::new(0.42874018118069736972e-3) * t36283;
    let t36286 = t3360 * t31349 * t4284;
    (t36274, t36276, t36279, t36284, t36286)
}
