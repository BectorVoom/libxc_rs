//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta1011 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3469;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3470;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3471;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3472;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3473;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3474;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3475;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta1011<F: Float>(t63586: F, t63589: F, t63592: F, t63596: F, t63600: F, t63607: F, t63609: F, t63612: F, t63615: F, t63618: F, t63620: F, t63622: F, t63625: F, t63628: F, t63633: F, t63636: F, t63638: F, t63641: F, t63644: F, t63647: F, t63649: F, t63653: F, t63656: F, t63660: F, t63662: F, t63665: F, t63668: F, t63670: F, t63673: F, t63676: F, t63679: F, t63681: F, t63683: F, t63685: F, t63820: F, t63826: F, t63833: F, t63835: F, t63894: F, t63898: F, t63906: F, t63916: F, t63918: F, t63920: F, t63923: F, t63925: F, t63927: F, t63929: F, t63934: F, t63937: F, t63940: F, t63943: F, t64327: F, t64329: F, t64488: F, t64491: F, t64493: F, t64496: F, t64498: F, t64500: F, t64503: F, t64507: F, t64509: F, t64335: F, t64338: F, t64340: F, t64342: F, t64344: F, t64346: F, t64404: F, t64512: F, t64521: F, t64523: F, t64527: F, t64529: F, t64531: F, t2986: F, t63902: F, t973: F, t981: F, t19468: F, t3022: F, t19021: F, t974: F, t2988: F, t41235: F, t41238: F, t6189: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t65389 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3469::<F>(t63586, t63589, t63592, t63596, t63600, t63607, t63609, t63612, t63615, t63618, t63620, t63622, t63625);
        let t65391 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3470::<F>(t63628, t63633, t63636, t63638, t63641, t63644, t63647, t63649, t63653, t63656, t63660, t63662);
        let t65392 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3471::<F>(t63665, t63668, t63670, t63673, t63676, t63679, t63681, t63683, t63685, t63820, t63826, t63833, t63835);
        let t65395 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3472::<F>(t63894, t63898, t63906, t63916, t63918, t63920, t63923, t63925, t63927, t63929, t63934, t63937);
        let t65396 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3473::<F>(t63940, t63943, t64327, t64329, t64488, t64491, t64493, t64496, t64498, t64500, t64503, t64507, t64509);
        let t65398 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3474::<F>(t64335, t64338, t64340, t64342, t64344, t64346, t64404, t64512, t64521, t64523, t64527, t64529, t64531);
        let (t65402, t65404, t65408, t65413) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3475::<F>(t2986, t63902, t973, t981, t19468, t3022, t19021, t974, t2988, t41235, t41238, t6189);
    (t65389, t65391, t65392, t65395, t65396, t65398, t65402, t65404, t65408, t65413)
}
