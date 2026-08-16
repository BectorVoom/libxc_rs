//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 947/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk947<F: Float>(t1369: F, t31176: F, t3876: F, t8466: F, t1998: F, t22845: F, t3734: F, t59: F, t3719: F, t6926: F, t22804: F, t31156: F) -> (F, F, F, F, F) {
    let t113987 = t31176 * t1369;
    let t113989 = t8466 * t3876;
    let t113993 = t22845 * t1998 * t59 * t3734;
    let t113997 = t6926 * t1998 * t59 * t3719;
    let t114000 = t22804 * t31156;
    (t113987, t113989, t113993, t113997, t114000)
}
