//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2889;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2890;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2891;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta822(t17279: f64, t699: f64, t17240: f64, t17243: f64, t136: f64, t2826: f64, t59715: f64, t10304: f64, t59751: f64, t59719: f64, t59706: f64, t41880: f64, t59711: f64, t59748: f64, t59753: f64, t59757: f64, t59759: f64, t59761: f64, t59765: f64, t59769: f64, t60079: f64, t60158: f64, t60185: f64, t60214: f64, t60242: f64, t60279: f64, t60300: f64, t17191: f64, t942: f64, t2929: f64, t5769: f64, t10820: f64, t14344: f64, t17355: f64, t17366: f64, t2900: f64, t2925: f64, t2933: f64, t42020: f64, t42123: f64, t4449: f64, t5762: f64, t5775: f64, t5791: f64, t60033: f64, t60035: f64, t60037: f64, t60039: f64, t60041: f64, t60044: f64, t60047: f64, t60050: f64, t60053: f64, t60056: f64, t943: f64, t951: f64, t952: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2889(t17279, t699, t17240, t17243, t136, t2826, t59715, t10304, t59751, t59719, t59706, t41880, t59711);
        let t60329 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2890(t59748, t59753, t59757, t59759, t59761, t59765, t59769, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let (t60332, t60338, t60343) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2891(t60079, t60158, t60185, t60214, t60242, t60279, t60300, t60329, t17191, t942, t2929, t5769);
        let t60346 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2892(t10820, t14344, t17355, t17366, t2900, t2925, t2933, t42020, t42123, t4449, t5762, t5775, t5791, t60033, t60035, t60037, t60039, t60041, t60044, t60047, t60050, t60053, t60056, t60332, t60338, t60343, t943, t951, t952);
    (t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327, t60332, t60346)
}
