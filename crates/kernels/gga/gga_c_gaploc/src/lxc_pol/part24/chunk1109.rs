//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1109/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1109<F: Float>(t549: F, t7069: F, t7390: F, t5638: F, t822: F, t9419: F, t20671: F, t22624: F, t28831: F, t825: F, t969: F, t2013: F, t9851: F) -> (F, F, F, F, F) {
    let t28854 = F::new(0.11916829983950142223e0) * t7390 * t549 * t7069;
    let t28856 = t822 * t5638 * t9419;
    let t28859 = F::new(0.51123901271894332902e0) * t28856 * t20671 * t22624;
    let t28861 = t825 * t969 * t28831;
    let t28862 = F::new(0.38342925953920749676e0) * t28861;
    let t28864 = F::new(0.17041300423964777634e0) * t2013 * t9851;
    (t28854, t28856, t28859, t28862, t28864)
}
