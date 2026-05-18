//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1448/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1448<F: Float>(t15008: F, t15122: F, t1554: F, t3980: F, t47639: F, t47654: F, t47659: F, t5233: F, t53510: F, t53909: F, t5434: F, t55795: F, t55797: F, t59214: F, t59218: F) -> F {
    let t60275 = F::new(32.0) / F::new(3.0) * t15008 * t5233 - F::new(16.0) / F::new(9.0) * t53909 - F::new(2.0) / F::new(9.0) * t47639 + F::new(0.31013857721884116596e-1) * t3980 * t15122 * t5434 - F::new(176.0) / F::new(27.0) * t47654 + F::new(2.0) / F::new(3.0) * t55795 - F::new(16.0) / F::new(3.0) * t55797 - t59214 - t59218 - F::new(4.0) / F::new(9.0) * t47659 - F::new(0.10337952573961372198e-1) * t3980 * t53510 * t1554;
    t60275
}
