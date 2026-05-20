//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1495/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1495<F: Float>(t105880: F, t117218: F, t117544: F, t117932: F, t118374: F, t1509: F, t21864: F, t31035: F, t31149: F, t31287: F, t31420: F, t31433: F, t31439: F, t31443: F, t4287: F, t5907: F, t5911: F, t5915: F, t661: F, t665: F, t8258: F, t8267: F, t8311: F, t8315: F) -> F {
    let t118728 = -F::new(25.0) / F::new(18.0) * t8258 * t31433 * t31420 + F::new(5.0) / F::new(6.0) * t8258 * t8315 * t4287 * t1509 - F::new(5.0) / F::new(6.0) * t117544 * t8315 * t118374 - F::new(3.0) / F::new(4.0) * t31035 * t8311 * t105880 + F::new(5.0) / F::new(12.0) * t8258 * t8315 * t5915 * t661 - F::new(25.0) / F::new(18.0) * t8258 * t31433 * t31439 + F::new(25.0) / F::new(54.0) * t8267 * t117932 * t31443 + F::new(5.0) / F::new(18.0) * t8258 * t31149 * t5907 * t665 + F::new(5.0) / F::new(108.0) * t8267 * t117218 * t5907 * t661 + F::new(5.0) / F::new(18.0) * t31287 * t31149 * t21864 + F::new(5.0) / F::new(12.0) * t8258 * t8315 * t5911 * t665;
    t118728
}
