//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1496/1505 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1496<F: Float>(t31027: F, t31633: F, t31640: F, t625: F, t105872: F, t116919: F, t117183: F, t117184: F, t117186: F, t117976: F, t117978: F, t118009: F, t118011: F, t31035: F, t31149: F, t5891: F, t5911: F, t661: F, t8267: F, t8311: F, t8315: F) -> F {
    let t118733 = t31027 * t31633;
    let t118744 = t625 * t31640;
    let t118746 = -F::new(5.0) / F::new(36.0) * t8267 * t31149 * t5911 * t661 - t117976 + t117978 - F::new(20.0) / F::new(9.0) * t118733 + F::new(3.0) * t116919 * t8311 * t105872 - F::new(5.0) / F::new(4.0) * t31035 * t8315 * t5891 * t661 + F::new(22.0) / F::new(9.0) * t117184 - F::new(55.0) / F::new(27.0) * t117186 + t117183 + t118009 - t118011 + F::new(40.0) / F::new(27.0) * t118744;
    t118746
}
