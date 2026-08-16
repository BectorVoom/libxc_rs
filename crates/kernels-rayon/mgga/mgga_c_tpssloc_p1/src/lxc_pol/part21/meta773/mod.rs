//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta773 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2675;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2676;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2677;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2678;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2679;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta773(t1388: f64, t5187: f64, t1307: f64, t5356: f64, t54392: f64, t54395: f64, t54398: f64, t54400: f64, t15904: f64, t20077: f64, t20085: f64, t3734: f64, t3918: f64, t39463: f64, t39468: f64, t39472: f64, t5126: f64, t5161: f64, t25: f64, t54402: f64, t2: f64, t584: f64, t606: f64, t11987: f64, t15989: f64, t16557: f64, t19606: f64, t19611: f64, t21: f64, t2249: f64, t3665: f64, t3704: f64, t39861: f64, t5170: f64, t53825: f64, t5397: f64, t6305: f64, t9: f64, t9212: f64, zeta_threshold: f64, t28: f64, t1081: f64, t12000: f64, t16003: f64, t18196: f64, t19618: f64, t19623: f64, t3231: f64, t3673: f64, t3711: f64, t39877: f64, t5178: f64, t53852: f64, t5966: f64, t6312: f64, t54405: f64, t12466: f64, t1297: f64, t15868: f64, t1799: f64, t193: f64, t19577: f64, t19596: f64, t19994: f64, t3719: f64, t3914: f64, t3919: f64, t39476: f64, t5160: f64, t55191: f64, t55266: f64, t6301: f64, t6347: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56202, t56203, t56207, t56208, t56212) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2675(t1388, t5187, t1307, t5356, t54392, t54395, t54398, t54400, t15904, t20077, t20085, t3734, t3918, t39463, t39468, t39472, t5126, t5161);
        let (t56219, t56226, t56247) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2676(t25, t54402, t2, t584, t606, t11987, t15989, t16557, t19606, t19611, t21, t2249, t3665, t3704, t39861, t5170, t53825, t5397, t6305, t9, t9212, zeta_threshold);
        let (t56252, t56273) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2677(t28, t1081, t2, t584, t12000, t16003, t18196, t19618, t19623, t21, t3231, t3673, t3711, t39877, t5178, t53852, t5966, t6312, t9, t9212, zeta_threshold);
        let t56275 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2678(t56247, t56273);
        let (t56279, t56294) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2679(t54405, t12466, t1297, t15868, t15904, t1799, t193, t19577, t19596, t19994, t20077, t3719, t3914, t3918, t3919, t39476, t5126, t5160, t55191, t55266, t56219, t56275, t6301, t6347);
    (t56202, t56203, t56207, t56208, t56212, t56219, t56226, t56252, t56275, t56279, t56294)
}
