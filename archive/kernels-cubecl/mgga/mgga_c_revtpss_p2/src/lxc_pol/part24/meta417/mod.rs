//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1363;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1364;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta417<F: Float>(t1086: F, t11200: F, t3090: F, t16565: F, t994: F, t42859: F, t42862: F, t342: F, t3145: F, t368: F, t42871: F, t42872: F, t1035: F, t357: F, t3057: F, t4980: F, t3286: F, t4995: F, t3143: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t43291, t43341, t43347, t43351) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1363::<F>(t1086, t11200, t3090, t16565, t994, t42859, t42862, t342, t3145, t368, t42871);
        let (t43352, t43401, t43402, t43438, t43446, t43456, t43471) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1364::<F>(t42872, t43351, t1035, t42859, t342, t357, t3057, t4980, t11200, t3286, t4995, t3143);
    (t43291, t43341, t43347, t43351, t43352, t43401, t43402, t43438, t43446, t43456, t43471)
}
