//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 947/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk947<F: Float>(t32540: F, t32574: F, t118: F, t1448: F, t2033: F, t28286: F, t28196: F, t10301: F, t8619: F, t10309: F, t644: F, t8621: F, t8622: F, t136: F, t7342: F, t2247: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32575 = t32540 + t32574;
    let t32576 = t118 * t32575;
    let t32577 = t2033 * t1448;
    let t32578 = t28286 * t32577;
    let t32580 = 2.0 * t28196 * t32578;
    let t32581 = t10301 * t8619;
    let t32584 = t10309 * t8619;
    let t32586 = t8621 * t8622 * t644;
    let t32589 = t7342 * t136;
    let t32590 = t2247 * t32589;
    (t32575, t32576, t32577, t32578, t32580, t32581, t32584, t32586, t32589, t32590)
}
