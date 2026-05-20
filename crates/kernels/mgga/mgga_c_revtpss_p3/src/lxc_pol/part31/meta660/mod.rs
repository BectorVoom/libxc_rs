//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta660 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2235;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2236;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2237;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2238;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta660<F: Float>(t28182: F, t7898: F, t29499: F, t7235: F, t2014: F, t29498: F, t32737: F, t27137: F, t7732: F, t2322: F, t29502: F, t4254: F, t5517: F, t651: F, t7741: F, t101417: F, t7900: F, t109035: F, t109038: F, t109039: F, t109041: F, t109043: F, t109045: F, t1518: F, t27830: F, t29986: F, t30116: F, t33602: F, t4293: F, t649: F, t196: F, t197: F, t22525: F, t2035: F, t22496: F, t25082: F, t33651: F, t29576: F, t22475: F, t7312: F, t2034: F, t73407: F, t30122: F, t32113: F, t1448: F, t6781: F, t28196: F, t98495: F, t1353: F, t28197: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t109047, t109049, t109052, t109054, t109058, t109060) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2235::<F>(t28182, t7898, t29499, t7235, t2014, t29498, t32737, t27137, t7732, t2322, t29502, t4254);
        let t109075 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2236::<F>(t5517, t651, t7741, t101417, t2014, t7900, t109035, t109038, t109039, t109041, t109043, t109045, t109047, t109049, t109052, t109054, t109058, t109060, t1518, t2322, t27830, t29986, t30116, t33602, t4254, t4293, t649);
        let (t109078, t109081, t109087, t109090) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2237::<F>(t196, t197, t22525, t2035, t22496, t25082, t33651, t29576, t7235, t2014, t22475, t7312);
        let (t109092, t109095, t109099, t109103) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2238::<F>(t2014, t2034, t73407, t25082, t30122, t32113, t1448, t6781, t28196, t98495, t1353, t28197);
    (t109075, t109078, t109081, t109087, t109090, t109092, t109095, t109099, t109103)
}
