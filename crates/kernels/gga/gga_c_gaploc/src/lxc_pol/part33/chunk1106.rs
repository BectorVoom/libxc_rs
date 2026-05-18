//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1106/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1106<F: Float>(t14667: F, t22045: F, t2365: F, t549: F, t7069: F, t7390: F, t5638: F, t822: F, t9419: F, t20671: F, t22624: F, t28831: F, t825: F, t969: F) -> (F, F, F, F, F) {
    let t28851 = F::new(0.59584149919750711116e-1) * t14667 * t2365 * t22045;
    let t28854 = F::new(0.11916829983950142223e0) * t7390 * t549 * t7069;
    let t28856 = t822 * t5638 * t9419;
    let t28859 = F::new(0.51123901271894332902e0) * t28856 * t20671 * t22624;
    let t28861 = t825 * t969 * t28831;
    (t28851, t28854, t28856, t28859, t28861)
}
