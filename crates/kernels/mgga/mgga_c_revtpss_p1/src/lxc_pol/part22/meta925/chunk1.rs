//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3148/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3148<F: Float>(t17225: F, t3647: F, t11262: F, t1261: F, t5303: F, t3711: F, t5298: F, t127: F, t17352: F) -> (F, F, F, F) {
    let t56734 = t3647 * t17225;
    let t56739 = t1261 * t11262 * t5303;
    let t56742 = t3711 * t11262 * t5298;
    let t56756 = t127 * t17352;
    (t56734, t56739, t56742, t56756)
}
