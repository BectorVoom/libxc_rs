//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 777/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk777<F: Float>(t1317: F, t3853: F, t3829: F, t4140: F, t5536: F, t9278: F, t9308: F, t9316: F, t9320: F, t9325: F, t9329: F, t9333: F, t9365: F, t9374: F, t9376: F, t9389: F, t9391: F, t9394: F) -> (F, F) {
    let t9395 = t1317 * t3853;
    let t9396 = F::cast_from(12.0_f64) * t9395;
    let t9397 = F::cast_from(18.0_f64) * t3829 * t4140 * t5536 - t9278 + t9308 + t9316 + t9320 - t9325 + t9329 + t9333 + t9365 - t9374 - t9376 - t9389 - t9391 + t9394 + t9396;
    (t9396, t9397)
}
