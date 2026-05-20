//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 954/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk954<F: Float>(t28063: F, t651: F, t22496: F, t8717: F, t25082: F, t1469: F, t25129: F, t25132: F, t25137: F, t4181: F, t4186: F, t6968: F) -> (F, F, F, F) {
    let t28065 = F::new(2.0) * t651 * t28063;
    let t28067 = t8717 * t22496;
    let t28069 = F::new(3.0) * t25082 * t28067;
    let t28076 = -F::new(20.0) / F::new(9.0) * t25129 * t1469 + F::new(5.0) / F::new(18.0) * t25132 * t4181 + F::new(5.0) / F::new(6.0) * t6968 * t4186 - t25137;
    (t28065, t28067, t28069, t28076)
}
