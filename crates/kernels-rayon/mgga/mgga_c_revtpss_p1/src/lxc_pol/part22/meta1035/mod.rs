//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1035 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3620;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3621;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1035(t68284: f64, t68338: f64, t68379: f64, t68466: f64, t68501: f64, t68526: f64, t68564: f64, t68595: f64, t1179: f64, t1188: f64, t1196: f64, t20397: f64, t3531: f64, t1187: f64, t5180: f64, t16997: f64, t58672: f64, t20567: f64, t300: f64, t1198: f64, t20400: f64, t3539: f64, t5501: f64, t5184: f64, t58665: f64, t1189: f64, t20382: f64, t3495: f64, t20472: f64, t3498: f64, t198: f64, t336: f64, t3801: f64, t68243: f64, t68245: f64, t68247: f64, t68250: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68598, t68602, t68604) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3620(t68284, t68338, t68379, t68466, t68501, t68526, t68564, t68595, t1179, t1188, t1196, t20397, t3531);
        let (t68605, t68608, t68611, t68613, t68614, t68621) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3621(t1187, t5180, t16997, t58672, t20567, t300, t1198, t20400, t3539, t5501, t5184, t58665);
        let (t68625, t68628, t68629) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3622(t1189, t1196, t20382, t3495, t20472, t3498, t198, t336, t3801, t68243, t68245, t68247, t68250, t68602, t68604, t68608, t68611, t68613, t68614, t68621);
    (t68598, t68602, t68604, t68605, t68608, t68611, t68613, t68621, t68625, t68628, t68629)
}
