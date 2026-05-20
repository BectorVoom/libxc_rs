//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1804;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta511<F: Float>(t233: F, t30379: F, t1957: F, t225: F, t2061: F, t5977: F, t2723: F, t25416: F, t231: F, t7076: F, t1558: F, t7997: F) -> (F, F, F, F, F, F, F, F) {
        let (t30380, t30381, t30384, t30391, t30392, t30395, t30396, t30400) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1804::<F>(t233, t30379, t1957, t225, t2061, t5977, t2723, t25416, t231, t7076, t1558, t7997);
    (t30380, t30381, t30384, t30391, t30392, t30395, t30396, t30400)
}
