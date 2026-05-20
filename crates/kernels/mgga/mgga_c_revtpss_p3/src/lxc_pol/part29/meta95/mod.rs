//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta95 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk583;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk584;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk585;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk586;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk587;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk588;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk589;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta95<F: Float>(t2098: F, t561: F, t2097: F, t545: F, t2028: F, t2027: F, t213: F, t532: F, t1450: F, t118: F, t2014: F, t2052: F, t2056: F, t2089: F, t2093: F, t508: F, t569: F, t651: F, t3: F, param_d: F, t117: F, t2055: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t2099, t2102) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk583::<F>(t2098, t561, t2097, t545);
        let t2103 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk584::<F>(t2028, t2102);
        let t2106 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk585::<F>(t2027, t2099, t2103, t213);
        let t2107 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk586::<F>(t2106, t532);
        let t2108 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk587::<F>(t1450, t2107);
        let (t2110, t2111, t2113) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk588::<F>(t118, t2014, t2052, t2056, t2089, t2093, t2108, t508, t569, t651, t3, param_d);
        let t2115 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk589::<F>(t117, t2055);
    (t2099, t2102, t2103, t2106, t2107, t2108, t2110, t2111, t2113, t2115)
}
