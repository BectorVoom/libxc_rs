//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta492 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1783;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1784;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1785;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1786;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1787;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta492<F: Float>(t14587: F, t28425: F, t26497: F, t4481: F, t26550: F, t27349: F, t14495: F, t27312: F, t212: F, t7997: F, t780: F, t689: F, t2067: F, t25391: F, t26541: F, t26545: F, t26557: F, t26558: F, t26561: F, t26564: F, t26578: F, t27199: F, t27275: F, t27353: F, t7415: F, t28358: F, t28397: F, t28424: F, t892: F, t2411: F, t8019: F, t198: F, t2075: F, t1940: F, t2071: F, t2255: F, t1468: F, t2403: F, t26425: F, t26585: F, t27160: F, t27166: F, t27169: F, t27173: F, t27376: F, t27385: F, t27387: F, t27391: F, t27395: F, t27402: F, t28291: F, t30: F, t605: F, t7010: F, t7092: F, t7428: F, t7432: F, t7749: F, t7787: F, t8020: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t28426, t28434, t28436, t28439, t28442, t28447, t28448, t28449) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1783::<F>(t14587, t28425, t26497, t4481, t26550, t27349, t14495, t27312, t212, t7997, t780, t689);
        let t28453 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1784::<F>(t2067, t25391, t26541, t26545, t26557, t26558, t26561, t26564, t26578, t27199, t27275, t27353, t28426, t28434, t28436, t28439, t28442, t28449, t7415);
        let (t28455, t28456) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1785::<F>(t28358, t28397, t28424, t28453, t892);
        let t28460 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1786::<F>(t2411, t8019);
        let t28472 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1787::<F>(t198, t2075);
        let (t28490, t28491) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1788::<F>(t1940, t2071, t2255, t1468, t2403, t26425, t26585, t27160, t27166, t27169, t27173, t27376, t27385, t27387, t27391, t27395, t27402, t28291, t28456, t28460, t28472, t30, t605, t7010, t7092, t7428, t7432, t7749, t7787, t8020);
    (t28426, t28436, t28439, t28442, t28447, t28448, t28455, t28456, t28460, t28472, t28490, t28491)
}
