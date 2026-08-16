//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1195/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1195<F: Float>(t1459: F, t34012: F, t1916: F, t32375: F, t1518: F, t572: F, t670: F, t8460: F, t32374: F, t4292: F, t5795: F, t8614: F) -> (F, F, F, F, F) {
    let t127453 = F::cast_from(6.0_f64) * t1459 * t34012;
    let t127455 = F::cast_from(6.0_f64) * t1916 * t32375;
    let t127459 = F::cast_from(6.0_f64) * t572 * t670 * t8460 * t1518;
    let t127462 = F::cast_from(6.0_f64) * t572 * t32374 * t4292;
    let t127495 = F::cast_from(3.0_f64) * t5795 * t8614;
    (t127453, t127455, t127459, t127462, t127495)
}
