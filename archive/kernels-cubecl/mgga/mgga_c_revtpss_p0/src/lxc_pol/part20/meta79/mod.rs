//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta79 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk485;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk486;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk487;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk488;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta79<F: Float>(t2398: F, t707: F, t150: F, t2389: F, t190: F, t198: F, t206: F, t890: F, t892: F, t45: F, t57: F, t261: F, t2258: F, t706: F, t2251: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F, t125: F, t215: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2400, t2401, t2402, t2403) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk485::<F>(t2398, t707, t150, t2389, t190, t198, t206);
        let (t2404, t2408) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk486::<F>(t890, t892);
        let (t2410, t2411, t2414, t2416, t2430) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk487::<F>(t45, t57, t261, t190, t2258, t706, t2251, t766, t80, t770, t83, zeta_threshold);
        let t2434 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk488::<F>(t125, t215);
    (t2400, t2401, t2402, t2403, t2404, t2408, t2410, t2411, t2414, t2416, t2430, t2434)
}
