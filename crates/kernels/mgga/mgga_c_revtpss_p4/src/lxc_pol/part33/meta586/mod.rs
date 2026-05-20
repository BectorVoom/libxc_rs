//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta586 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1999;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2000;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta586<F: Float>(t10982: F, t1949: F, t9646: F, t2471: F, t25355: F, t10985: F, t25422: F, t25335: F, t9303: F, t1959: F, t41117: F, t68: F, t785: F, t251: F, t281: F, t25410: F, t7078: F, t2453: F, t2458: F, t7049: F, t1950: F, t2769: F, t786: F, t25404: F, t40270: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t93206, t93207, t93210, t93224, t93231, t93238) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1999::<F>(t10982, t1949, t9646, t2471, t25355, t10985, t25422, t25335, t9303, t1959, t41117, t68, t785);
        let (t93240, t93242, t93252, t93261, t93272) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2000::<F>(t251, t281, t93238, t25410, t7078, t2453, t2458, t7049, t1950, t2769, t786, t25404, t40270);
    (t93206, t93207, t93210, t93224, t93231, t93238, t93240, t93242, t93252, t93261, t93272)
}
