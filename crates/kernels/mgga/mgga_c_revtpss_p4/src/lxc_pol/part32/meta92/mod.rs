//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta92 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk564;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk565;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk566;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk567;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk568;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk569;
use chunk6::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk570;
use chunk7::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk571;
use chunk8::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk572;
use chunk9::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk573;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta92<F: Float>(t5: F, t1923: F, t2048: F, t117: F, t114: F, t1934: F, t508: F, t1943: F, t1947: F, t225: F, t257: F, t233: F, t1957: F, t1956: F, t213: F, t892: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t2051 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk564::<F>(t5, t1923, t2048);
        let t2052 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk565::<F>(t117, t2051);
        let t2055 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk566::<F>(t114, t1934);
        let t2056 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk567::<F>(t2055, t508);
        let t2061 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk568::<F>(t1943, t1947);
        let t2062 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk569::<F>(t2061, t225);
        let (t2063, t2066) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk570::<F>(t2062, t257, t2061, t233);
        let t2067 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk571::<F>(t1957, t2066);
        let t2070 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk572::<F>(t1956, t2063, t2067, t213);
        let t2071 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk573::<F>(t2070, t892);
    (t2051, t2052, t2055, t2056, t2061, t2062, t2063, t2066, t2067, t2070, t2071)
}
