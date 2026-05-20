//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1491;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1492;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1493;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta402<F: Float>(t1053: F, t11788: F, t11783: F, t3215: F, t11817: F, t3211: F, t1025: F, t1026: F, t2434: F, t371: F, t11901: F, t993: F, t225: F, t366: F, t11792: F, t11951: F, t3224: F, t11809: F, t127: F, t11782: F, t1065: F, t3133: F, t372: F, t1043: F, t1045: F, t11165: F, t3181: F, t11156: F, t1011: F, t1028: F, t11637: F, t11774: F, t15700: F, t15701: F, t16012: F, t16226: F, t16229: F, t41248: F, t41263: F, t4786: F, t4919: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t42265, t42268, t42270, t42274, t42277) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1491::<F>(t1053, t11788, t11783, t3215, t11817, t3211, t1025, t1026, t2434, t371, t11901, t993);
        let (t42278, t42279, t42282, t42284, t42288, t42290) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1492::<F>(t225, t42277, t366, t11792, t3215, t11951, t3224, t1025, t11809, t127, t371, t1053, t11782);
        let (t42300, t42309, t42315, t42320) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1493::<F>(t1065, t3133, t372, t1043, t1045, t11165, t3181, t11156, t1011, t1028, t11637, t11774, t15700, t15701, t16012, t16226, t16229, t41248, t41263, t42279, t42282, t42284, t42288, t42290, t4786, t4919);
    (t42265, t42268, t42270, t42274, t42277, t42278, t42300, t42309, t42315, t42320)
}
