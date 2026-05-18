//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1198/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1198<F: Float>(t25894: F, t96186: F, t94398: F, t122: F, t72: F, t7506: F, t25900: F, t25904: F, t26231: F, t94802: F, t2435: F, t26355: F) -> (F, F, F, F, F, F) {
    let t96187 = t25894 * t96186;
    let t96188 = t96187 * t94398;
    let t96191 = t7506 * t72 * t122;
    let t96192 = t96191 * t25900;
    let t96193 = t25904 * t96192;
    let t96195 = t94802 * t26231;
    let t96197 = t2435 * t26355;
    (t96188, t96191, t96192, t96193, t96195, t96197)
}
