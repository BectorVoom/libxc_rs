//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3234/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3234<F: Float>(t1471: F, t1487: F, t1494: F, t21769: F, t21805: F, t22718: F, t22739: F, t4188: F, t4191: F, t4217: F, t4218: F, t4238: F, t5819: F, t5855: F, t5869: F, t607: F, t628: F, t71: F, t77: F, t85: F, t85125: F) -> F {
    let t85141 = t21769 * t1494 / F::new(8.0) + t5855 * t4238 / F::new(8.0) + t4218 * t5869 / F::new(8.0) + t1487 * t21805 / F::new(8.0) + t628 * t22739 / F::new(24.0) + t71 * t77 * t85125 / F::new(24.0) - t5819 * t4217 * t85 / F::new(4.0) - t4188 * t5869 / F::new(4.0) - t4191 * t5869 / F::new(4.0) - t1471 * t21805 / F::new(4.0) - t607 * t22718 * t85 / F::new(12.0);
    t85141
}
