//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 681/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk681<F: Float>(t603: F, t607: F, t43: F, t48: F, t624: F, t49: F, t606: F, t613: F, t72: F, t1927: F) -> (F, F, F, F, F) {
    let t6963 = t603 * t607;
    let t6968 = t43 * t48;
    let t6971 = 8.0 / 3.0 * t624;
    let t6972 = -8.0 / 3.0 * t613 * t49 + 5.0 / 6.0 * t6968 * t606 + t6971;
    let t6973 = t6972 * t72;
    let t6974 = t6973 * t1927;
    (t6963, t6968, t6972, t6973, t6974)
}
