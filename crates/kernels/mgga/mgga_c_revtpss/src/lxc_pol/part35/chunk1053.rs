//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1053/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1053<F: Float>(t4173: F, t5819: F, t22738: F, t76: F, t38: F, t85037: F, t1518: F, t5876: F, t1501: F, t5920: F, t22633: F, t93: F, t22589: F, t94982: F, t25826: F, t75833: F) -> (F, F, F, F, F, F, F, F) {
    let t114322 = t4173 * t5819;
    let t114343 = t76 * t22738;
    let t114349 = t85037 * t38;
    let t114373 = t5876 * t1518;
    let t114378 = t1501 * t5920;
    let t114385 = t93 * t22633;
    let t114394 = t94982 * t22589;
    let t114396 = t25826 * t75833;
    (t114322, t114343, t114349, t114373, t114378, t114385, t114394, t114396)
}
