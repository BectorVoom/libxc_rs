//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 858/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk858<F: Float>(t2349: F, t5895: F, t100: F, t5823: F, t1479: F, t1509: F, t2357: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t97: F, tau1: F) -> (F, F, F, F, F, F) {
    let t5896 = t2349 * t5895;
    let t5899 = t100 * t5823;
    let t5902 = tau1 * t1479;
    let t5907 = t1509 * t1509;
    let t5908 = t2357 * t5907;
    let t5911 = -t5823;
    let t5912 = t108 * t5911;
    let t5915 = F::new(10.0) / F::new(9.0) * t97 * t5896 + F::new(5.0) / F::new(3.0) * t97 * t5899 + F::new(40.0) / F::new(9.0) * t5902 * t109 - F::new(50.0) / F::new(9.0) * t1507 * t1510 + F::new(10.0) / F::new(9.0) * t105 * t5908 + F::new(5.0) / F::new(3.0) * t105 * t5912;
    (t5896, t5899, t5902, t5907, t5911, t5915)
}
