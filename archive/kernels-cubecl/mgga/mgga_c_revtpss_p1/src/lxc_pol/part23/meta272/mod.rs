//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1485;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1486;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta272<F: Float>(t231: F, t281: F, t68: F, t836: F, t10535: F, t2783: F, t860: F, t786: F, t760: F, t9323: F, t9318: F, t2609: F, t717: F, t162: F, t9544: F) -> (F, F, F, F, F, F, F, F) {
        let (t10538, t10539, t10541, t10542, t10552, t10554, t10563) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1485::<F>(t231, t281, t68, t836, t10535, t2783, t860, t786, t760, t9323, t9318, t2609, t717);
        let t10565 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1486::<F>(t162, t9544);
    (t10538, t10539, t10541, t10542, t10552, t10554, t10563, t10565)
}
