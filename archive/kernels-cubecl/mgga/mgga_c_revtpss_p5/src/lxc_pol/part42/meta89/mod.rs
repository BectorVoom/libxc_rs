//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta89 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk520;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk521;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk522;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk523;
use chunk4::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk524;
use chunk5::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta89<F: Float>(t1916: F, t1918: F, t572: F, t573: F, t76: F, t84: F, t198: F, t207: F, t159: F, t215: F, t104: F, t655: F, t114: F, t109: F, t69: F, t508: F, t569: F, t1312: F, t651: F, t3: F, param_d: F, t117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1921, t1927, t1940, t1941, t2194) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk520::<F>(t1916, t1918, t572, t573, t76, t84, t198, t207, t159, t215, t104, t655);
        let (t2195, t2198) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk521::<F>(t114, t109, t2194, t69);
        let t2199 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk522::<F>(t2198, t508);
        let t2201 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk523::<F>(t2198, t569);
        let (t2204, t2205, t2207) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk524::<F>(t1312, t2199, t2201, t651, t3, param_d);
        let t2209 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk525::<F>(t117, t2198);
    (t1921, t1927, t1940, t1941, t2195, t2198, t2199, t2201, t2204, t2205, t2207, t2209)
}
