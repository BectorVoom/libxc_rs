//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1054/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1054<F: Float>(t22628: F, t6998: F, t1907: F, t6836: F, t1955: F, t22964: F, t22829: F, t26028: F, t27932: F, t85776: F, t22890: F, t22849: F, t7252: F, t22877: F, t94516: F, t22881: F) -> (F, F, F, F, F, F, F, F, F) {
    let t114398 = t6998 * t22628;
    let t114452 = t6836 * t1907;
    let t114485 = t1955 * t22964;
    let t114521 = t26028 * t22829;
    let t114525 = t27932 * t85776;
    let t114527 = t26028 * t22890;
    let t114541 = t7252 * t22849;
    let t114543 = t94516 * t22877;
    let t114545 = t26028 * t22881;
    (t114398, t114452, t114485, t114521, t114525, t114527, t114541, t114543, t114545)
}
