//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta90 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk550;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk551;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk552;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk553;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk554;
use chunk5::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk555;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta90<F: Float>(t1916: F, t1918: F, t572: F, t573: F, t38: F, t603: F, t76: F, t84: F, t112: F, t68: F, t198: F, t207: F, t159: F, t215: F, t218: F, t816: F, t234: F, t64: F, t213: F, t248: F, t209: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1921, t1923) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk550::<F>(t1916, t1918, t572, t573, t38, t603);
        let t1927 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk551::<F>(t76, t84);
        let (t1934, t1940) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk552::<F>(t112, t68, t198, t207);
        let t1941 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk553::<F>(t159, t215);
        let (t1943, t1945) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk554::<F>(t1941, t218, t816, t234, t64);
        let (t1946, t1947, t1954, t1955) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk555::<F>(t1945, t213, t248, t209, t785);
    (t1921, t1923, t1927, t1934, t1940, t1941, t1943, t1945, t1946, t1947, t1954, t1955)
}
