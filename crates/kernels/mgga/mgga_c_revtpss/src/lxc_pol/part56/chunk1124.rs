//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1124/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1124<F: Float>(t33703: F, t689: F, t120151: F, t120005: F, t33711: F, t846: F, t1568: F, t31805: F, t817: F, t8485: F, t31845: F, t33695: F) -> (F, F, F, F, F) {
    let t126102 = t33703 * t689;
    let t126103 = t120151 * t126102;
    let t126105 = t120005 * t126102;
    let t126108 = t33711 * t846;
    let t126110 = t31805 * t1568;
    let t126112 = t126110 * t8485 * t817;
    let t126121 = t33695 * t31845;
    (t126103, t126105, t126108, t126112, t126121)
}
