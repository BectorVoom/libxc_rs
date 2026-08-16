//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta774 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2680;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2681;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta774(t25: f64, t54408: f64, t54411: f64, t12061: f64, t15937: f64, t16557: f64, t19547: f64, t19552: f64, t21: f64, t2249: f64, t3664: f64, t3665: f64, t39419: f64, t5134: f64, t5397: f64, t54347: f64, t56226: f64, t584: f64, t606: f64, t6305: f64, t9: f64, t9212: f64, zeta_threshold: f64, t28: f64, t1081: f64, t12072: f64, t15952: f64, t18196: f64, t19559: f64, t19564: f64, t3231: f64, t3672: f64, t3673: f64, t39436: f64, t5142: f64, t54370: f64, t56252: f64, t5966: f64, t6312: f64, t157: f64, t182: f64, t1390: f64, t20063: f64, t54412: f64, t39491: f64, t12466: f64, t1307: f64, t16148: f64, t3918: f64, t39483: f64, t39490: f64, t39496: f64, t5122: f64, t5126: f64, t6330: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t56298, t56299, t56323) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2680(t25, t54408, t54411, t12061, t15937, t16557, t19547, t19552, t21, t2249, t3664, t3665, t39419, t5134, t5397, t54347, t56226, t584, t606, t6305, t9, t9212, zeta_threshold);
        let t56347 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2681(t28, t1081, t12072, t15952, t18196, t19559, t19564, t21, t3231, t3672, t3673, t39436, t5142, t54370, t56252, t584, t5966, t6312, t9, t9212, zeta_threshold);
        let (t56349, t56351, t56362, t56363, t56364) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2682(t157, t56323, t56347, t182, t1390, t20063, t54412, t39491, t12466, t1307, t16148, t3918, t39483, t39490, t39496, t5122, t5126, t56298, t56299, t6330);
    (t56298, t56299, t56349, t56351, t56362, t56363, t56364)
}
