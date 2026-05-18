//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1156/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1156<F: Float>(t1976: F, t3042: F, t7145: F, t988: F, t993: F, t378: F, t8521: F, t995: F, t1043: F, t1089: F, t7146: F, t342: F, t7135: F) -> (F, F, F, F, F, F, F, F) {
    let t25620 = t1976 * t3042;
    let t25621 = t7145 * t25620;
    let t25624 = t988 * t988;
    let t25625 = t25624 * t993;
    let t25626 = t25625 * t378;
    let t25629 = t995 * t8521;
    let t25631 = t7146 * t1043 * t1089;
    let t25634 = t342 * t7135;
    (t25620, t25621, t25624, t25625, t25626, t25629, t25631, t25634)
}
