//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1720;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1721;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta465<F: Float>(t26291: F, t26374: F, t532: F, t1450: F, t1310: F, t18163: F, t2014: F, t2056: F, t2089: F, t2093: F, t2320: F, t2322: F, t2328: F, t2372: F, t26154: F, t26162: F, t26210: F, t26218: F, t26223: F, t4151: F, t4254: F, t508: F, t649: F, t651: F, t7235: F, t7357: F, t7359: F, t7367: F, t7374: F, t7378: F, t7474: F, t7489: F, t7539: F, t7315: F, t7536: F, t25089: F, t7488: F, t2107: F, t25802: F, t7373: F, t116: F, t7356: F, t2106: F, t4147: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t26375, t26376, t26377, t26379) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1720::<F>(t26291, t26374, t532, t1450, t1310, t18163, t2014, t2056, t2089, t2093, t2320, t2322, t2328, t2372, t26154, t26162, t26210, t26218, t26223, t4151, t4254, t508, t649, t651, t7235, t7357, t7359, t7367, t7374, t7378, t7474, t7489, t7539);
        let (t26380, t26383, t26392, t26396, t26399) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1721::<F>(t7315, t7536, t25089, t7488, t2107, t25802, t1310, t7373, t116, t7356);
        let t26405 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1722::<F>(t2106, t4147);
    (t26375, t26376, t26377, t26379, t26380, t26383, t26392, t26396, t26399, t26405)
}
