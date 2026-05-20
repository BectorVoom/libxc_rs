//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta320 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1317;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta320<F: Float>(t10535: F, t10538: F, t2783: F, t860: F, t786: F, t2801: F, t231: F, t2645: F, t268: F, t675: F, t2798: F, t760: F, t9323: F, t9318: F, t2251: F, t750: F, t2611: F, t2398: F, t2615: F, t2609: F, t717: F, t162: F, t9544: F, t158: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t10539, t10542, t10543, t10548, t10552) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1317::<F>(t10535, t10538, t2783, t860, t786, t2801, t231, t2645, t268, t675, t2798, t760, t9323);
        let (t10554, t10556, t10561, t10563, t10566) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1318::<F>(t760, t9318, t2251, t750, t2611, t2398, t2615, t2609, t717, t162, t9544, t158);
    (t10539, t10542, t10543, t10548, t10552, t10554, t10556, t10561, t10563, t10566)
}
