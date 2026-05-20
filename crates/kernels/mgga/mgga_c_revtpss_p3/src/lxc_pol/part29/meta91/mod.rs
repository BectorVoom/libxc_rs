//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta91 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk552;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk553;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk554;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk555;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk556;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk557;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta91<F: Float>(t117: F, t1518: F, t1916: F, t572: F, t573: F, t38: F, t603: F, t76: F, t84: F, t112: F, t68: F, t198: F, t207: F, t159: F, t215: F, t218: F, t816: F, t234: F, t64: F) -> (F, F, F, F, F, F, F, F, F) {
        let t1918 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk552::<F>(t117, t1518);
        let (t1921, t1923) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk553::<F>(t1916, t1918, t572, t573, t38, t603);
        let t1927 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk554::<F>(t76, t84);
        let (t1934, t1940) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk555::<F>(t112, t68, t198, t207);
        let t1941 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk556::<F>(t159, t215);
        let (t1943, t1945) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk557::<F>(t1941, t218, t816, t234, t64);
    (t1918, t1921, t1923, t1927, t1934, t1940, t1941, t1943, t1945)
}
