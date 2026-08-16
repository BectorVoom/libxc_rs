//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta406 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1501;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1502;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1503;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1504;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta406(t1063: f64, t11169: f64, t247: f64, t3109: f64, t1011: f64, t11758: f64, t140: f64, t11823: f64, t11821: f64, t41270: f64, t11828: f64, t11144: f64, t3252: f64, t1012: f64, t1015: f64, t1066: f64, t11829: f64, t11853: f64, t11913: f64, t3188: f64, t3241: f64, t39443: f64, t39457: f64, t41271: f64, t41318: f64, t11852: f64, t126: f64, t11145: f64, t11679: f64, t11710: f64, t3091: f64, t11247: f64, t11249: f64, t3105: f64, t3223: f64, t11960: f64, t351: f64, t361: f64, t369: f64, t1041: f64, t11262: f64, t3135: f64, t1033: f64, t1036: f64, t1038: f64, t1042: f64, t1047: f64, t1065: f64, t1068: f64, t11173: f64, t11233: f64, t11281: f64, t11286: f64, t11656: f64, t11845: f64, t11983: f64, t2853: f64, t3059: f64, t3106: f64, t3127: f64, t3130: f64, t3181: f64, t4837: f64, t906: f64, t11160: f64, t11620: f64, t73: f64, t3153: f64, t12166: f64, t15905: f64, t994: f64, t11631: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42496, t42499, t42506, t42508, t42516, t42518) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1501(t1063, t11169, t247, t3109, t1011, t11758, t140, t11823, t11821, t41270, t11828, t11144, t3252);
        let t42529 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1502(t1011, t1012, t1015, t1063, t1066, t11829, t11853, t11913, t247, t3188, t3241, t39443, t39457, t41271, t41318, t42496, t42499, t42506, t42508, t42516, t42518);
        let (t42537, t42546, t42550, t42571) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1503(t11852, t126, t1063, t11145, t247, t11679, t11710, t3091, t11247, t11249, t3105, t3223);
        let t42602 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1504(t11960, t351, t361, t369, t1041, t11262, t3135, t1033, t1036, t1038, t1042, t1047, t1065, t1068, t11173, t11233, t11281, t11286, t11656, t11845, t11983, t2853, t3059, t3106, t3127, t3130, t3181, t42571, t4837, t906);
        let (t42606, t42610, t42615, t42621, t42622) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1505(t1063, t11160, t247, t3109, t11620, t73, t3153, t12166, t15905, t994, t11631, t999);
    (t42529, t42537, t42546, t42550, t42602, t42606, t42610, t42615, t42621, t42622)
}
