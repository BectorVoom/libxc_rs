//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta497 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1851;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta497<F: Float>(t5: F, t30: F, t265: F, t393: F, t26798: F, t117: F, t2126: F, t2327: F, t25743: F, t2129: F, t2258: F, t25459: F, t45: F, t606: F, t7594: F, t2138: F, t3650: F, dens_threshold: F, rho0: F, zeta_threshold: F, t2139: F, t3655: F, t1256: F, t7610: F, t3670: F, t3666: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26799, t26800, t26804, t26809, t26816, t26817) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1851::<F>(t5, t30, t265, t393, t26798, t117, t2126, t2327, t25743, t2129, t2258, t25459, t45, t606, t7594, t2138, t3650, dens_threshold, rho0, zeta_threshold);
        let (t26821, t26822, t26824, t26827) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1852::<F>(t2139, t3655, t1256, t7610, t2138, t3670, t3666);
    (t26799, t26800, t26804, t26809, t26816, t26817, t26821, t26822, t26824, t26827)
}
