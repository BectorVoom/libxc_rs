//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta96 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk656;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk657;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk658;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk659;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk660;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta96<F: Float>(t57: F, t2251: F, t2258: F, t2382: F, t81: F, t2381: F, t162: F, t187: F, t205: F, t262: F, t775: F, zeta_threshold: F, t705: F, t716: F, t707: F, t150: F, t190: F, t198: F, t206: F, t890: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2389, t2390, t2392, t2393, t2394) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk656::<F>(t57, t2251, t2258, t2382, t81, t2381, t162, t187, t205, t262, t775, zeta_threshold);
        let t2398 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk657::<F>(t705, t716);
        let (t2400, t2401, t2402, t2403) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk658::<F>(t2398, t707, t150, t2389, t190, t198, t206);
        let t2404 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk659::<F>(t890, t892);
        let t2408 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk660::<F>(t890);
    (t2389, t2390, t2392, t2393, t2394, t2398, t2400, t2401, t2402, t2403, t2404, t2408)
}
