//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 136/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk136<F: Float>(t41: F, t425: F, t68: F, t63: F, t390: F, t393: F, t398: F, t388: F, t71: F) -> (F, F, F, F, F, F, F, F, F) {
    let t426 = t41 * t425;
    let t430 = t68 * t68;
    let t431 = F::new(1.0) / t430;
    let t432 = t63 * t431;
    let t434 = F::new(0.516475e0) * t390;
    let t435 = F::new(0.2103875e0) * t393;
    let t436 = F::new(0.104195e0) * t398;
    let t437 = -F::new(0.1176575e1) * t388 - t434 - t435 - t436;
    let t438 = F::new(1.0) / t71;
    (t426, t430, t431, t432, t434, t435, t436, t437, t438)
}
