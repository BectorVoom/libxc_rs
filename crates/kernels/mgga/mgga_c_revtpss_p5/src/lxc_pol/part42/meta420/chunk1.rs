//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1481/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1481<F: Float>(t5911: F, t8315: F, t31035: F, t31134: F, t31415: F, t31427: F, t31437: F, t31626: F, t31629: F, t31633: F, t31636: F, t31640: F, t31643: F, t31646: F, t69: F, t8258: F, t8267: F) -> (F, F) {
    let t31649 = t8315 * t5911;
    let t31652 = -t31134 - F::new(4.0) / F::new(3.0) * t31415 - F::new(10.0) / F::new(9.0) * t31427 + F::new(10.0) / F::new(9.0) * t31437 - F::new(3.0) / F::new(4.0) * t31035 * t31626 - F::new(5.0) / F::new(6.0) * t8258 * t31629 + F::new(5.0) / F::new(6.0) * t8258 * t31633 + t8258 * t31636 / F::new(4.0) - F::new(5.0) / F::new(9.0) * t69 * t31640 + F::new(25.0) / F::new(36.0) * t8267 * t31643 - F::new(5.0) / F::new(36.0) * t8267 * t31646 - F::new(5.0) / F::new(24.0) * t8267 * t31649;
    (t31649, t31652)
}
