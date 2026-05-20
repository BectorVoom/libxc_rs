//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1475/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1475<F: Float>(t31439: F, t8315: F, t1509: F, t661: F, t31149: F, t2: F, t31035: F, t31134: F, t31135: F, t31137: F, t31287: F, t31415: F, t31417: F, t31421: F, t31424: F, t31427: F, t31430: F, t31434: F, t31437: F, t8258: F, t8267: F) -> (F, F, F, F, F) {
    let t31440 = t8315 * t31439;
    let t31443 = t1509 * t661;
    let t31444 = t31149 * t31443;
    let t31447 = t8315 * t2;
    let t31450 = -t31134 - F::new(2.0) / F::new(3.0) * t31135 + F::new(5.0) / F::new(9.0) * t31137 - F::new(2.0) / F::new(3.0) * t31415 - F::new(3.0) / F::new(4.0) * t31035 * t31417 + F::new(5.0) / F::new(12.0) * t8258 * t31421 + t8258 * t31424 / F::new(4.0) - F::new(5.0) / F::new(9.0) * t31427 - F::new(5.0) / F::new(12.0) * t8258 * t31430 + F::new(25.0) / F::new(72.0) * t8267 * t31434 + F::new(5.0) / F::new(9.0) * t31437 + F::new(5.0) / F::new(12.0) * t8258 * t31440 - F::new(5.0) / F::new(36.0) * t8267 * t31444 + F::new(5.0) / F::new(24.0) * t31287 * t31447;
    (t31440, t31443, t31444, t31447, t31450)
}
