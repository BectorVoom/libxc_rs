//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta92 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk558;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk559;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk560;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk561;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk562;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta92<F: Float>(t1945: F, t213: F, t248: F, t209: F, t785: F, t251: F, t1032: F, t867: F, t196: F, t511: F, t197: F, t1941: F, t533: F, t816: F, t546: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t1946, t1947, t1954, t1955) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk558::<F>(t1945, t213, t248, t209, t785);
        let t1956 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk559::<F>(t1955, t251);
        let t1957 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk560::<F>(t1032, t867);
        let (t2013, t2014) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk561::<F>(t196, t511, t197);
        let (t2016, t2018) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk562::<F>(t1941, t533, t816, t546, t64);
    (t1946, t1947, t1954, t1955, t1956, t1957, t2013, t2014, t2016, t2018)
}
