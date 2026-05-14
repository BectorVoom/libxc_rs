//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1097/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1097<F: Float>(t22496: F, t25082: F, t37318: F, t128353: F, t2056: F, t128355: F, t34258: F, t7367: F, t111176: F, t28196: F, t32577: F, t8697: F, t8995: F, t28199: F, t122647: F, t27154: F) -> (F, F, F, F, F, F, F) {
    let t128513 = 3.0 * t25082 * t37318 * t22496;
    let t128517 = 2.0 * t128353 * t2056;
    let t128519 = 2.0 * t128355 * t2056;
    let t128521 = 2.0 * t34258 * t7367;
    let t128528 = 2.0 * t28196 * t111176 * t32577;
    let t128529 = t8697 * t8995;
    let t128531 = 2.0 * t128529 * t28199;
    let t128533 = 3.0 * t122647 * t27154;
    (t128513, t128517, t128519, t128521, t128528, t128531, t128533)
}
