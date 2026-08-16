//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1109;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1110;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1111;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta320<F: Float>(t22633: F, t508: F, t1501: F, t5883: F, t10271: F, t10273: F, t10275: F, t10278: F, t10280: F, t10282: F, t10284: F, t10287: F, t10289: F, t10291: F, t10295: F, t1497: F, t5816: F, t5872: F, t1927: F, t5825: F, t1486: F, t5819: F, t22603: F, t30: F, t33: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22634, t22639, t22648) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1109::<F>(t22633, t508, t1501, t5883, t10271, t10273, t10275, t10278, t10280, t10282, t10284, t10287, t10289, t10291, t10295);
        let (t22656, t22659, t22662, t22665, t22670) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1110::<F>(t1497, t5816, t5872, t1927, t5825, t1486, t5819, t22603);
        let t22671 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1111::<F>(t30, t33, t22670, zeta_threshold);
    (t22634, t22639, t22648, t22656, t22659, t22662, t22665, t22670, t22671)
}
