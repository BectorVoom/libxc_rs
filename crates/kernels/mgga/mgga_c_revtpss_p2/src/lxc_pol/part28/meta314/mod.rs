//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta314 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1317;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1318;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta314<F: Float>(t10509: F, t123: F, t2465: F, t213: F, t2760: F, t215: F, t231: F, t268: F, t836: F, t2798: F, t2722: F, t675: F, t251: F, t4503: F, t786: F, t2723: F, t2453: F, t2797: F, t281: F, t68: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t10510, t10511, t10513, t10519, t10521) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1317::<F>(t10509, t123, t2465, t213, t2760, t215, t231, t268, t836, t2798, t2722, t675);
        let (t10524, t10529, t10533, t10535, t10538) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1318::<F>(t10521, t231, t268, t2798, t251, t4503, t786, t2723, t2453, t2797, t281, t68, t836);
    (t10510, t10511, t10513, t10519, t10524, t10529, t10533, t10535, t10538)
}
