//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1056/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1056<F: Float>(t32540: F, t32574: F, t118: F, t1448: F, t2033: F, t28286: F, t28196: F, t7003: F, t7359: F, t7316: F, t8698: F, t196: F, t197: F, t7484: F) -> (F, F, F, F, F, F, F, F) {
    let t32575 = t32540 + t32574;
    let t32576 = t118 * t32575;
    let t32577 = t2033 * t1448;
    let t32578 = t28286 * t32577;
    let t32580 = F::new(2.0) * t28196 * t32578;
    let t32619 = F::new(2.0) * t7359 * t7003;
    let t32620 = t8698 * t7316;
    let t32626 = t7484 * t196 * t197;
    (t32575, t32576, t32577, t32578, t32580, t32619, t32620, t32626)
}
