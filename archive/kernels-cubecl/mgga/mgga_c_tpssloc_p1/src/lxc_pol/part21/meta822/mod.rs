//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta822 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2889;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2890;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2891;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2892;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta822<F: Float>(t17279: F, t699: F, t17240: F, t17243: F, t136: F, t2826: F, t59715: F, t10304: F, t59751: F, t59719: F, t59706: F, t41880: F, t59711: F, t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F, t60079: F, t60158: F, t60185: F, t60214: F, t60242: F, t60279: F, t60300: F, t17191: F, t942: F, t2929: F, t5769: F, t10820: F, t14344: F, t17355: F, t17366: F, t2900: F, t2925: F, t2933: F, t42020: F, t42123: F, t4449: F, t5762: F, t5775: F, t5791: F, t60033: F, t60035: F, t60037: F, t60039: F, t60041: F, t60044: F, t60047: F, t60050: F, t60053: F, t60056: F, t943: F, t951: F, t952: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2889::<F>(t17279, t699, t17240, t17243, t136, t2826, t59715, t10304, t59751, t59719, t59706, t41880, t59711);
        let t60329 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2890::<F>(t59748, t59753, t59757, t59759, t59761, t59765, t59769, t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327);
        let (t60332, t60338, t60343) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2891::<F>(t60079, t60158, t60185, t60214, t60242, t60279, t60300, t60329, t17191, t942, t2929, t5769);
        let t60346 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2892::<F>(t10820, t14344, t17355, t17366, t2900, t2925, t2933, t42020, t42123, t4449, t5762, t5775, t5791, t60033, t60035, t60037, t60039, t60041, t60044, t60047, t60050, t60053, t60056, t60332, t60338, t60343, t943, t951, t952);
    (t60308, t60310, t60312, t60315, t60318, t60321, t60324, t60327, t60332, t60346)
}
