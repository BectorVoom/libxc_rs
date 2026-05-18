//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1213/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1213<F: Float>(t26265: F, t9671: F, t26230: F, t94403: F, t25904: F, t4078: F, t689: F, t7492: F, t2097: F, t9898: F, t94589: F, t96279: F) -> (F, F, F, F, F, F) {
    let t96423 = t26265 * t9671;
    let t96431 = t26230 * t94403;
    let t96432 = t25904 * t96431;
    let t96437 = t689 * t7492 * t4078;
    let t96443 = t2097 * t9898;
    let t96456 = t94589 * t96279;
    (t96423, t96431, t96432, t96437, t96443, t96456)
}
