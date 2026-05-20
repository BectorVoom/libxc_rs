//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta116 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk678;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk679;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk680;
use chunk3::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk681;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta116<F: Float>(t2619: F, t755: F, t72: F, t752: F, t757: F, t2492: F, t2596: F, t745: F, t760: F, t123: F, t192: F, t676: F, t762: F, t2392: F, t2400: F, t2402: F, t2416: F, t2498: F, t2518: F, t2522: F, t2525: F, t2527: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2614: F, t2617: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2621, t2622, t2623, t2624, t2626) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk678::<F>(t2619, t755, t72, t752, t757, t2492, t2596, t745);
        let (t2628, t2629) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk679::<F>(t2626, t760, t123, t192);
        let t2630 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk680::<F>(t676, t762);
        let (t2632, t2633) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk681::<F>(t2629, t2630, t2392, t2400, t2402, t2416, t2498, t2518, t2522, t2525, t2527, t2562, t2569, t2579, t2587, t2610, t2614, t2617, t2621, t2624, t2628);
    (t2621, t2622, t2623, t2624, t2626, t2628, t2629, t2630, t2632, t2633)
}
