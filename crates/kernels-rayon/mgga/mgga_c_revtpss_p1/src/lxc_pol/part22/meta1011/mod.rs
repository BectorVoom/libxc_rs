//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1011 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3469;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3470;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3471;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3472;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3473;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3474;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1011(t63586: f64, t63589: f64, t63592: f64, t63596: f64, t63600: f64, t63607: f64, t63609: f64, t63612: f64, t63615: f64, t63618: f64, t63620: f64, t63622: f64, t63625: f64, t63628: f64, t63633: f64, t63636: f64, t63638: f64, t63641: f64, t63644: f64, t63647: f64, t63649: f64, t63653: f64, t63656: f64, t63660: f64, t63662: f64, t63665: f64, t63668: f64, t63670: f64, t63673: f64, t63676: f64, t63679: f64, t63681: f64, t63683: f64, t63685: f64, t63820: f64, t63826: f64, t63833: f64, t63835: f64, t63894: f64, t63898: f64, t63906: f64, t63916: f64, t63918: f64, t63920: f64, t63923: f64, t63925: f64, t63927: f64, t63929: f64, t63934: f64, t63937: f64, t63940: f64, t63943: f64, t64327: f64, t64329: f64, t64488: f64, t64491: f64, t64493: f64, t64496: f64, t64498: f64, t64500: f64, t64503: f64, t64507: f64, t64509: f64, t64335: f64, t64338: f64, t64340: f64, t64342: f64, t64344: f64, t64346: f64, t64404: f64, t64512: f64, t64521: f64, t64523: f64, t64527: f64, t64529: f64, t64531: f64, t2986: f64, t63902: f64, t973: f64, t981: f64, t19468: f64, t3022: f64, t19021: f64, t974: f64, t2988: f64, t41235: f64, t41238: f64, t6189: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t65389 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3469(t63586, t63589, t63592, t63596, t63600, t63607, t63609, t63612, t63615, t63618, t63620, t63622, t63625);
        let t65391 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3470(t63628, t63633, t63636, t63638, t63641, t63644, t63647, t63649, t63653, t63656, t63660, t63662);
        let t65392 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3471(t63665, t63668, t63670, t63673, t63676, t63679, t63681, t63683, t63685, t63820, t63826, t63833, t63835);
        let t65395 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3472(t63894, t63898, t63906, t63916, t63918, t63920, t63923, t63925, t63927, t63929, t63934, t63937);
        let t65396 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3473(t63940, t63943, t64327, t64329, t64488, t64491, t64493, t64496, t64498, t64500, t64503, t64507, t64509);
        let t65398 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3474(t64335, t64338, t64340, t64342, t64344, t64346, t64404, t64512, t64521, t64523, t64527, t64529, t64531);
        let (t65402, t65404, t65408, t65413) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3475(t2986, t63902, t973, t981, t19468, t3022, t19021, t974, t2988, t41235, t41238, t6189);
    (t65389, t65391, t65392, t65395, t65396, t65398, t65402, t65404, t65408, t65413)
}
