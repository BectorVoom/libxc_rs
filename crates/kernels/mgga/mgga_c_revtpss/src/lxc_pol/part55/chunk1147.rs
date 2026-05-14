//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1147/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1147<F: Float>(t127455: F, t127459: F, t127462: F, t129034: F, t129039: F, t129045: F, t129048: F, t129055: F, t129057: F, t129065: F, t1461: F, t2170: F, t28978: F, t34838: F, t7554: F, t8245: F) -> (F,) {
    let t131155 = 3.0 * t1461 * t34838 + 6.0 * t2170 * t28978 + 6.0 * t7554 * t8245 + t127455 + t127459 + t127462 + t129034 + t129039 + t129045 + t129048 + t129055 + t129057 + t129065;
    (t131155,)
}
