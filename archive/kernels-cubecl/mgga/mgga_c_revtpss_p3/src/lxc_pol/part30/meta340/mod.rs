//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1354;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta340<F: Float>(t2912: F, t698: F, t11132: F, t240: F, t624: F, t281: F, t283: F, t2909: F, t3252: F, t276: F, t285: F, t273: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11326, t11334, t11335, t11337, t11338, t11339, t11341, t11354, t11358) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1354::<F>(t2912, t698, t11132, t240, t624, t281, t283, t2909, t3252, t276, t285, t273);
    (t11326, t11334, t11335, t11337, t11338, t11339, t11341, t11354, t11358)
}
