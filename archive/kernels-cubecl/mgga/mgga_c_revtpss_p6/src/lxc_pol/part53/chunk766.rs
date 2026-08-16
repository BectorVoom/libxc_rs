//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 766/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk766<F: Float>(t1802: F, t2137: F, t467: F, t1782: F, t1791: F, t1797: F, t1808: F, t464: F, t484: F, t7606: F, t7607: F, t7613: F, t7618: F, t7622: F, t7624: F, t8172: F, t8177: F) -> (F, F, F) {
    let t8184 = t2137 * t1802;
    let t8185 = t467 * t8184;
    let t8190 = -t8172 * t464 / F::cast_from(36.0_f64) + t7606 - t7607 * t1782 / F::cast_from(288.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t8177 * t484 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t1791 + F::cast_from(0.42874018118069736972e-3_f64) * t7618 * t1797 - F::cast_from(0.22866142996303859718e-2_f64) * t8185 * t484 + t7622 - F::cast_from(0.28582678745379824648e-3_f64) * t7624 * t1808;
    (t8184, t8185, t8190)
}
