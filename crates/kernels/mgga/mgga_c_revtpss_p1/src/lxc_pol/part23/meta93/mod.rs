//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk642;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk643;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta93<F: Float>(t2440: F, t780: F, t2439: F, t212: F, t860: F, t689: F, t779: F, t887: F, t211: F, t784: F, t209: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk642::<F>(t2440, t780, t2439, t212, t860, t689, t779, t887, t211, t784);
        let t2453 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk643::<F>(t209, t2452);
    (t2441, t2443, t2444, t2445, t2446, t2448, t2449, t2452, t2453)
}
