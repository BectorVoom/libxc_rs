//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk579;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk580;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk581;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk582;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk583;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk584;
use chunk6::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk585;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta91<F: Float>(t1950: F, t257: F, t209: F, t785: F, t251: F, t1032: F, t867: F, t1949: F, t233: F, t213: F, t892: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t1951, t1954, t1955) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk579::<F>(t1950, t257, t209, t785);
        let t1956 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk580::<F>(t1955, t251);
        let t1957 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk581::<F>(t1032, t867);
        let t1958 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk582::<F>(t1949, t233);
        let t1959 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk583::<F>(t1957, t1958);
        let t1962 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk584::<F>(t1951, t1956, t1959, t213);
        let t1963 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk585::<F>(t1962, t892);
    (t1951, t1954, t1955, t1956, t1957, t1958, t1959, t1962, t1963)
}
