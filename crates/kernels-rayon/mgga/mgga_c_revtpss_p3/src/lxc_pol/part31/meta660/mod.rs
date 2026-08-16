//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2235;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2236;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2237;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta660(t28182: f64, t7898: f64, t29499: f64, t7235: f64, t2014: f64, t29498: f64, t32737: f64, t27137: f64, t7732: f64, t2322: f64, t29502: f64, t4254: f64, t5517: f64, t651: f64, t7741: f64, t101417: f64, t7900: f64, t109035: f64, t109038: f64, t109039: f64, t109041: f64, t109043: f64, t109045: f64, t1518: f64, t27830: f64, t29986: f64, t30116: f64, t33602: f64, t4293: f64, t649: f64, t196: f64, t197: f64, t22525: f64, t2035: f64, t22496: f64, t25082: f64, t33651: f64, t29576: f64, t22475: f64, t7312: f64, t2034: f64, t73407: f64, t30122: f64, t32113: f64, t1448: f64, t6781: f64, t28196: f64, t98495: f64, t1353: f64, t28197: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t109047, t109049, t109052, t109054, t109058, t109060) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2235(t28182, t7898, t29499, t7235, t2014, t29498, t32737, t27137, t7732, t2322, t29502, t4254);
        let t109075 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2236(t5517, t651, t7741, t101417, t2014, t7900, t109035, t109038, t109039, t109041, t109043, t109045, t109047, t109049, t109052, t109054, t109058, t109060, t1518, t2322, t27830, t29986, t30116, t33602, t4254, t4293, t649);
        let (t109078, t109081, t109087, t109090) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2237(t196, t197, t22525, t2035, t22496, t25082, t33651, t29576, t7235, t2014, t22475, t7312);
        let (t109092, t109095, t109099, t109103) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2238(t2014, t2034, t73407, t25082, t30122, t32113, t1448, t6781, t28196, t98495, t1353, t28197);
    (t109075, t109078, t109081, t109087, t109090, t109092, t109095, t109099, t109103)
}
