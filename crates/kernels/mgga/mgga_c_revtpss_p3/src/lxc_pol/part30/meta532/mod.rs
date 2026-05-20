//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta532<F: Float>(t30: F, t265: F, t393: F, t28271: F, t572: F, t1459: F, t7953: F, t116: F, t7741: F, t670: F, t117: F, t28042: F, t27754: F, t1469: F, t2129: F, t27408: F, t4186: F, t45: F, t606: F, t7594: F, t8161: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28273, t28275, t28276, t28277, t28279, t28280, t28282, t28998, t29005) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1947::<F>(t30, t265, t393, t28271, t572, t1459, t7953, t116, t7741, t670, t117, t28042, t27754, t1469, t2129, t27408, t4186, t45, t606, t7594, t8161, dens_threshold, rho0, zeta_threshold);
    (t28273, t28275, t28276, t28277, t28279, t28280, t28282, t28998, t29005)
}
