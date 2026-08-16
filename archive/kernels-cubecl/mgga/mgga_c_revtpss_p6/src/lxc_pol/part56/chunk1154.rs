//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1154/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1154<F: Float>(t32374: F, t4292: F, t572: F, t26123: F, t7741: F, t28042: F, t7330: F, t1459: F, t34004: F, t105823: F, t7002: F, t8453: F) -> (F, F, F, F, F, F) {
    let t127462 = F::cast_from(6.0_f64) * t572 * t32374 * t4292;
    let t127465 = F::cast_from(12.0_f64) * t572 * t26123 * t7741;
    let t127468 = F::cast_from(12.0_f64) * t572 * t7330 * t28042;
    let t127472 = F::cast_from(6.0_f64) * t1459 * t34004;
    let t127480 = F::cast_from(12.0_f64) * t572 * t105823 * t7002;
    let t127489 = F::cast_from(6.0_f64) * t572 * t4292 * t8453;
    (t127462, t127465, t127468, t127472, t127480, t127489)
}
