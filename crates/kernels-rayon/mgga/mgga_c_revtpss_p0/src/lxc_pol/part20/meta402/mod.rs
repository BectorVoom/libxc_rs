//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1492;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta402(t1053: f64, t11788: f64, t11783: f64, t3215: f64, t11817: f64, t3211: f64, t1025: f64, t1026: f64, t2434: f64, t371: f64, t11901: f64, t993: f64, t225: f64, t366: f64, t11792: f64, t11951: f64, t3224: f64, t11809: f64, t127: f64, t11782: f64, t1065: f64, t3133: f64, t372: f64, t1043: f64, t1045: f64, t11165: f64, t3181: f64, t11156: f64, t1011: f64, t1028: f64, t11637: f64, t11774: f64, t15700: f64, t15701: f64, t16012: f64, t16226: f64, t16229: f64, t41248: f64, t41263: f64, t4786: f64, t4919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42265, t42268, t42270, t42274, t42277) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1491(t1053, t11788, t11783, t3215, t11817, t3211, t1025, t1026, t2434, t371, t11901, t993);
        let (t42278, t42279, t42282, t42284, t42288, t42290) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1492(t225, t42277, t366, t11792, t3215, t11951, t3224, t1025, t11809, t127, t371, t1053, t11782);
        let (t42300, t42309, t42315, t42320) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1493(t1065, t3133, t372, t1043, t1045, t11165, t3181, t11156, t1011, t1028, t11637, t11774, t15700, t15701, t16012, t16226, t16229, t41248, t41263, t42279, t42282, t42284, t42288, t42290, t4786, t4919);
    (t42265, t42268, t42270, t42274, t42277, t42278, t42300, t42309, t42315, t42320)
}
