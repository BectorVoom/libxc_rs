//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3239/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3239<F: Float>(t1470: F, t1494: F, t21690: F, t21707: F, t21710: F, t21713: F, t21768: F, t22665: F, t22681: F, t22719: F, t38: F, t4182: F, t4238: F, t5820: F, t5830: F, t5869: F, t641: F, t85: F, t85255: F, t85295: F) -> F {
    let t85300 = -t1470 * t21768 * t85 / F::new(4.0) - t22681 * t641 / F::new(4.0) - t21707 * t1494 / F::new(2.0) - t21710 * t1494 / F::new(2.0) - t21713 * t1494 / F::new(2.0) - t5830 * t4238 / F::new(2.0) - t4182 * t5869 / F::new(4.0) + t22719 * t641 / F::new(24.0) - t22665 * t641 / F::new(4.0) - t21690 * t1494 / F::new(4.0) - t5820 * t4238 / F::new(4.0) + t38 * (t85255 + t85295) * t85 / F::new(24.0);
    t85300
}
