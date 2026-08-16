//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1501;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1502;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1503;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1504;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta406<F: Float>(t1063: F, t11169: F, t247: F, t3109: F, t1011: F, t11758: F, t140: F, t11823: F, t11821: F, t41270: F, t11828: F, t11144: F, t3252: F, t1012: F, t1015: F, t1066: F, t11829: F, t11853: F, t11913: F, t3188: F, t3241: F, t39443: F, t39457: F, t41271: F, t41318: F, t11852: F, t126: F, t11145: F, t11679: F, t11710: F, t3091: F, t11247: F, t11249: F, t3105: F, t3223: F, t11960: F, t351: F, t361: F, t369: F, t1041: F, t11262: F, t3135: F, t1033: F, t1036: F, t1038: F, t1042: F, t1047: F, t1065: F, t1068: F, t11173: F, t11233: F, t11281: F, t11286: F, t11656: F, t11845: F, t11983: F, t2853: F, t3059: F, t3106: F, t3127: F, t3130: F, t3181: F, t4837: F, t906: F, t11160: F, t11620: F, t73: F, t3153: F, t12166: F, t15905: F, t994: F, t11631: F, t999: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42496, t42499, t42506, t42508, t42516, t42518) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1501::<F>(t1063, t11169, t247, t3109, t1011, t11758, t140, t11823, t11821, t41270, t11828, t11144, t3252);
        let t42529 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1502::<F>(t1011, t1012, t1015, t1063, t1066, t11829, t11853, t11913, t247, t3188, t3241, t39443, t39457, t41271, t41318, t42496, t42499, t42506, t42508, t42516, t42518);
        let (t42537, t42546, t42550, t42571) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1503::<F>(t11852, t126, t1063, t11145, t247, t11679, t11710, t3091, t11247, t11249, t3105, t3223);
        let t42602 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1504::<F>(t11960, t351, t361, t369, t1041, t11262, t3135, t1033, t1036, t1038, t1042, t1047, t1065, t1068, t11173, t11233, t11281, t11286, t11656, t11845, t11983, t2853, t3059, t3106, t3127, t3130, t3181, t42571, t4837, t906);
        let (t42606, t42610, t42615, t42621, t42622) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1505::<F>(t1063, t11160, t247, t3109, t11620, t73, t3153, t12166, t15905, t994, t11631, t999);
    (t42529, t42537, t42546, t42550, t42602, t42606, t42610, t42615, t42621, t42622)
}
