//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3237/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3237<F: Float>(t1480: F, t21754: F, t21762: F, t21765: F, t22689: F, t22695: F, t22700: F, t4186: F, t4214: F, t44: F, t46090: F, t48: F, t56: F, t5843: F, t60: F, t60308: F, t60311: F, t614: F, t620: F, t76397: F, t77513: F) -> F {
    let t85255 = -F::new(5.0) / F::new(36.0) * t60308 * t77513 + F::new(5.0) / F::new(36.0) * t60311 * t77513 - t46090 + F::new(10.0) / F::new(81.0) * t614 * t22689 - F::new(20.0) / F::new(9.0) * t614 * t22695 + F::new(5.0) / F::new(6.0) * t44 * t48 * t76397 + F::new(3080.0) / F::new(81.0) * t22700 * t620 - F::new(220.0) / F::new(9.0) * t5843 * t4214 + F::new(20.0) / F::new(3.0) * t1480 * t21765 - F::new(5.0) / F::new(6.0) * t56 * t60 * t76397 - F::new(20.0) / F::new(9.0) * t1480 * t21762 + F::new(5.0) / F::new(36.0) * t56 * t21754 * t4186;
    t85255
}
