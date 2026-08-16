//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;
mod chunk9;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk570;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk571;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk572;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk573;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk574;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk575;
use chunk6::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk576;
use chunk7::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk577;
use chunk8::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk578;
use chunk9::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk579;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta90<F: Float>(t1925: F, t72: F, t76: F, t84: F, t5: F, t1923: F, t117: F, t114: F, t112: F, t68: F, t508: F, t651: F, t198: F, t207: F, t159: F, t215: F, t218: F, t816: F, t234: F, t64: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1926 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk570::<F>(t1925, t72);
        let t1927 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk571::<F>(t76, t84);
        let t1928 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk572::<F>(t1926, t1927);
        let t1931 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk573::<F>(t5, t1923, t1928);
        let t1932 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk574::<F>(t117, t1931);
        let t1936 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk575::<F>(t114, t112, t68);
        let t1937 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk576::<F>(t1936, t508);
        let (t1939, t1940) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk577::<F>(t1937, t651, t198, t207);
        let t1941 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk578::<F>(t159, t215);
        let (t1943, t1945) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk579::<F>(t1941, t218, t816, t234, t64);
    (t1926, t1927, t1928, t1931, t1932, t1936, t1937, t1939, t1940, t1941, t1943, t1945)
}
