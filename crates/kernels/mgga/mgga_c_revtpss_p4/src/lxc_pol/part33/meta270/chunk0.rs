//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1209/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1209<F: Float>(t3: F, t7690: F, t1461: F, t2170: F, t573: F, t7329: F, t7333: F, t7336: F, t38: F, t4173: F, t1497: F, t84: F, param_d: F) -> (F, F, F, F, F) {
    let t7691 = t3 * t7690;
    let t7696 = param_d * t7690;
    let t7700 = F::new(3.0) * t1461 * t2170 + t573 * t7696 + t7329 + t7333 + t7336;
    let t7702 = t4173 * t38;
    let t7705 = t84 * t1497;
    (t7691, t7696, t7700, t7702, t7705)
}
