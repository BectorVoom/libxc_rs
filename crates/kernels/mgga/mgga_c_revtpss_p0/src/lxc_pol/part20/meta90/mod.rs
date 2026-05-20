//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk528;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk529;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk530;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta90<F: Float>(t190: F, t2251: F, t2611: F, t606: F, t750: F, t706: F, t186: F, t215: F, t685: F, t755: F, t72: F, t752: F, t757: F, t2492: F, t2596: F, t745: F, t760: F, t123: F, t192: F, t676: F, t762: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2612, t2614, t2615, t2617, t2619) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk528::<F>(t190, t2251, t2611, t606, t750, t706, t186, t215, t685);
        let (t2621, t2622, t2624, t2626) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk529::<F>(t2619, t755, t72, t752, t757, t2492, t2596, t745);
        let (t2628, t2629) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk530::<F>(t2626, t760, t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk531::<F>(t676, t762);
    (t2612, t2614, t2615, t2617, t2619, t2621, t2622, t2624, t2626, t2628, t2629, t2630)
}
