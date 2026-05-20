//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta89 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk572;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk573;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk574;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk575;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk576;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk577;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk578;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta89<F: Float>(t114: F, t112: F, t68: F, t508: F, t651: F, t198: F, t207: F, t159: F, t215: F, t218: F, t816: F, t234: F, t64: F, t213: F, t248: F, t225: F, t257: F, t209: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1936 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk572::<F>(t114, t112, t68);
        let t1937 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk573::<F>(t1936, t508);
        let (t1939, t1940) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk574::<F>(t1937, t651, t198, t207);
        let t1941 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk575::<F>(t159, t215);
        let (t1943, t1945) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk576::<F>(t1941, t218, t816, t234, t64);
        let (t1946, t1949) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk577::<F>(t1945, t213, t248, t1943);
        let t1950 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk578::<F>(t1949, t225);
        let (t1951, t1954, t1955) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk579::<F>(t1950, t257, t209, t785);
    (t1936, t1937, t1939, t1940, t1941, t1945, t1946, t1949, t1950, t1951, t1954, t1955)
}
