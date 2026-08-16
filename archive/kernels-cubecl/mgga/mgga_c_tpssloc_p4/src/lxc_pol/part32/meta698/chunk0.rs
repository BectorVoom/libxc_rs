//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2175/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2175<F: Float>(t26322: F, t7708: F, t91202: F, t20004: F, t26309: F, t19945: F, t19981: F, t22833: F, t19994: F, t221: F, t26284: F, t19631: F, t1998: F, t236: F, t6926: F) -> (F, F, F, F, F, F) {
    let t97410 = t91202 * t7708 * t26322;
    let t97412 = t26309 * t20004;
    let t97414 = t26309 * t19945;
    let t97416 = t22833 * t19981;
    let t97419 = t26284 * t221 * t19994;
    let t97423 = t6926 * t1998 * t236 * t19631;
    (t97410, t97412, t97414, t97416, t97419, t97423)
}
