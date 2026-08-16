//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1720;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1721;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1722;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta465(t26291: f64, t26374: f64, t532: f64, t1450: f64, t1310: f64, t18163: f64, t2014: f64, t2056: f64, t2089: f64, t2093: f64, t2320: f64, t2322: f64, t2328: f64, t2372: f64, t26154: f64, t26162: f64, t26210: f64, t26218: f64, t26223: f64, t4151: f64, t4254: f64, t508: f64, t649: f64, t651: f64, t7235: f64, t7357: f64, t7359: f64, t7367: f64, t7374: f64, t7378: f64, t7474: f64, t7489: f64, t7539: f64, t7315: f64, t7536: f64, t25089: f64, t7488: f64, t2107: f64, t25802: f64, t7373: f64, t116: f64, t7356: f64, t2106: f64, t4147: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t26375, t26376, t26377, t26379) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1720(t26291, t26374, t532, t1450, t1310, t18163, t2014, t2056, t2089, t2093, t2320, t2322, t2328, t2372, t26154, t26162, t26210, t26218, t26223, t4151, t4254, t508, t649, t651, t7235, t7357, t7359, t7367, t7374, t7378, t7474, t7489, t7539);
        let (t26380, t26383, t26392, t26396, t26399) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1721(t7315, t7536, t25089, t7488, t2107, t25802, t1310, t7373, t116, t7356);
        let t26405 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1722(t2106, t4147);
    (t26375, t26376, t26377, t26379, t26380, t26383, t26392, t26396, t26399, t26405)
}
