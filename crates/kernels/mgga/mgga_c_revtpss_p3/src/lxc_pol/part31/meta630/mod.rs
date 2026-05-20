//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta630 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2084;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta630<F: Float>(t25569: F, t4817: F, t1659: F, t25576: F, t27489: F, t3111: F, t11940: F, t7131: F, t16158: F, t7132: F, t100007: F, t16094: F) -> (F, F, F, F, F, F) {
        let (t100097, t100114, t100117, t100121, t100132, t100135) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2084::<F>(t25569, t4817, t1659, t25576, t27489, t3111, t11940, t7131, t16158, t7132, t100007, t16094);
    (t100097, t100114, t100117, t100121, t100132, t100135)
}
