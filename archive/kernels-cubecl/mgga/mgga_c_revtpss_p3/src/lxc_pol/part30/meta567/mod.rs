//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2014;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2015;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta567<F: Float>(t2471: F, t25355: F, t10985: F, t25422: F, t25335: F, t9303: F, t25425: F, t689: F, t25431: F, t25411: F, t1959: F, t41117: F, t68: F, t785: F, t251: F, t281: F, t25410: F, t7078: F, t2453: F, t2458: F, t7049: F, t1950: F, t2769: F, t786: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t93207, t93210, t93224, t93226, t93228, t93231) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2014::<F>(t2471, t25355, t10985, t25422, t25335, t9303, t25425, t689, t25431, t25411, t1959, t41117);
        let (t93238, t93240, t93242, t93252, t93261) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2015::<F>(t68, t785, t251, t281, t25410, t7078, t2453, t2458, t7049, t1950, t2769, t786);
    (t93207, t93210, t93224, t93226, t93228, t93231, t93238, t93240, t93242, t93252, t93261)
}
