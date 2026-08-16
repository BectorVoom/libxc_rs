//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta492 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1783;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1784;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1785;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1786;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1787;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta492(t14587: f64, t28425: f64, t26497: f64, t4481: f64, t26550: f64, t27349: f64, t14495: f64, t27312: f64, t212: f64, t7997: f64, t780: f64, t689: f64, t2067: f64, t25391: f64, t26541: f64, t26545: f64, t26557: f64, t26558: f64, t26561: f64, t26564: f64, t26578: f64, t27199: f64, t27275: f64, t27353: f64, t7415: f64, t28358: f64, t28397: f64, t28424: f64, t892: f64, t2411: f64, t8019: f64, t198: f64, t2075: f64, t1940: f64, t2071: f64, t2255: f64, t1468: f64, t2403: f64, t26425: f64, t26585: f64, t27160: f64, t27166: f64, t27169: f64, t27173: f64, t27376: f64, t27385: f64, t27387: f64, t27391: f64, t27395: f64, t27402: f64, t28291: f64, t30: f64, t605: f64, t7010: f64, t7092: f64, t7428: f64, t7432: f64, t7749: f64, t7787: f64, t8020: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28426, t28434, t28436, t28439, t28442, t28447, t28448, t28449) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1783(t14587, t28425, t26497, t4481, t26550, t27349, t14495, t27312, t212, t7997, t780, t689);
        let t28453 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1784(t2067, t25391, t26541, t26545, t26557, t26558, t26561, t26564, t26578, t27199, t27275, t27353, t28426, t28434, t28436, t28439, t28442, t28449, t7415);
        let (t28455, t28456) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1785(t28358, t28397, t28424, t28453, t892);
        let t28460 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1786(t2411, t8019);
        let t28472 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1787(t198, t2075);
        let (t28490, t28491) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1788(t1940, t2071, t2255, t1468, t2403, t26425, t26585, t27160, t27166, t27169, t27173, t27376, t27385, t27387, t27391, t27395, t27402, t28291, t28456, t28460, t28472, t30, t605, t7010, t7092, t7428, t7432, t7749, t7787, t8020);
    (t28426, t28436, t28439, t28442, t28447, t28448, t28455, t28456, t28460, t28472, t28490, t28491)
}
