//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1438;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta465<F: Float>(t17306: F, t3754: F, t10308: F, t1466: F, t2246: F, t5812: F, t11064: F, t6075: F, t37: F, t5940: F, t2609: F, t5825: F, t706: F, t2611: F, t5819: F, t14440: F, t4311: F, t123: F, t2630: F, t5941: F, t18555: F, t2619: F, t18562: F, t2516: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t60019, t60224, t60673, t61033, t61037, t61090) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1438::<F>(t17306, t3754, t10308, t1466, t2246, t5812, t11064, t6075, t37, t5940, t2609, t5825, t706);
        let (t61165, t61180, t61247, t61282, t61294) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1439::<F>(t2609, t2611, t5819, t14440, t4311, t123, t2630, t5941, t18555, t2619, t18562, t2516);
    (t60019, t60224, t60673, t61033, t61037, t61090, t61165, t61180, t61247, t61282, t61294)
}
