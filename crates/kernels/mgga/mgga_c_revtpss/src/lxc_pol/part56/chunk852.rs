//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 852/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk852<F: Float>(t1203: F, t494: F, t247: F, t3719: F, t1032: F, t7642: F, t8925: F) -> (F, F, F, F) {
    let t33399 = t494 * t1203;
    let t33401 = t247 * t3719 * t33399;
    let t33404 = t7642 * t1032;
    let t33405 = t33404 * t8925;
    (t33399, t33401, t33404, t33405)
}
