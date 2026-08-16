//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1035 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3620;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3621;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3622;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1035<F: Float>(t68284: F, t68338: F, t68379: F, t68466: F, t68501: F, t68526: F, t68564: F, t68595: F, t1179: F, t1188: F, t1196: F, t20397: F, t3531: F, t1187: F, t5180: F, t16997: F, t58672: F, t20567: F, t300: F, t1198: F, t20400: F, t3539: F, t5501: F, t5184: F, t58665: F, t1189: F, t20382: F, t3495: F, t20472: F, t3498: F, t198: F, t336: F, t3801: F, t68243: F, t68245: F, t68247: F, t68250: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t68598, t68602, t68604) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3620::<F>(t68284, t68338, t68379, t68466, t68501, t68526, t68564, t68595, t1179, t1188, t1196, t20397, t3531);
        let (t68605, t68608, t68611, t68613, t68614, t68621) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3621::<F>(t1187, t5180, t16997, t58672, t20567, t300, t1198, t20400, t3539, t5501, t5184, t58665);
        let (t68625, t68628, t68629) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3622::<F>(t1189, t1196, t20382, t3495, t20472, t3498, t198, t336, t3801, t68243, t68245, t68247, t68250, t68602, t68604, t68608, t68611, t68613, t68614, t68621);
    (t68598, t68602, t68604, t68605, t68608, t68611, t68613, t68621, t68625, t68628, t68629)
}
