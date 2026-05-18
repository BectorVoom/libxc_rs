//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1205/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1205<F: Float>(t5795: F, t8611: F, t1459: F, t34012: F, t1916: F, t32375: F, t1518: F, t572: F, t670: F, t8460: F, t32374: F, t4292: F) -> (F, F, F, F, F) {
    let t127449 = F::new(6.0) * t5795 * t8611;
    let t127453 = F::new(6.0) * t1459 * t34012;
    let t127455 = F::new(6.0) * t1916 * t32375;
    let t127459 = F::new(6.0) * t572 * t670 * t8460 * t1518;
    let t127462 = F::new(6.0) * t572 * t32374 * t4292;
    (t127449, t127453, t127455, t127459, t127462)
}
