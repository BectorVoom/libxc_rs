//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3678/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3678<F: Float>(t12361: F, t20577: F, t20580: F, t44101: F, t20641: F, t12243: F, t20645: F, t1149: F, t20448: F, t3384: F, t20447: F, t3435: F) -> (F, F, F, F, F, F) {
    let t69581 = F::new(8.0) * t12361 * t20577;
    let t69583 = F::cast_from(0.19298375398431042081e3_f64) * t44101 * t20580;
    let t69585 = F::new(4.0) * t12361 * t20641;
    let t69587 = F::cast_from(0.32163958997385070134e2_f64) * t12243 * t20645;
    let t69590 = F::new(4.0) * t3384 * t20448 * t1149;
    let t69591 = t20447 * t3435;
    (t69581, t69583, t69585, t69587, t69590, t69591)
}
