//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1175/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1175<F: Float>(t110541: F, t25411: F, t110275: F, t93281: F, t6049: F, t689: F, t7384: F, t1580: F, t28447: F, t110502: F, t25387: F, t18797: F, t26497: F) -> (F, F, F, F, F, F) {
    let t110544 = t25411 * t110541;
    let t110572 = t93281 * t110275;
    let t110584 = t689 * t7384 * t6049;
    let t110591 = t689 * t28447 * t1580;
    let t110600 = t25387 * t110502;
    let t110613 = t26497 * t18797;
    (t110544, t110572, t110584, t110591, t110600, t110613)
}
