//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta999 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3391;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3392;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3393;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3394;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3395;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3396;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta999<F: Float>(t15386: F, t52508: F, t4732: F, t52452: F, t981: F, t2873: F, t6104: F, t2876: F, t15520: F, t4719: F, t19082: F, t3022: F, t19150: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t51921: F, t51923: F, t63238: F, t63240: F, t63242: F, t63246: F, t63250: F, t63255: F, t63260: F, t41246: F, t41281: F, t41285: F, t41287: F, t51937: F, t51942: F, t63266: F, t63268: F, t63274: F, t63276: F, t63278: F, t63281: F, t63285: F, t63290: F, t63293: F, t41307: F, t41361: F, t41363: F, t51967: F, t51973: F, t51978: F, t63299: F, t63304: F, t63308: F, t63311: F, t63315: F, t63320: F, t63325: F, t63328: F, t63332: F, t52033: F, t63336: F, t63338: F, t63340: F, t63342: F, t63346: F, t63351: F, t63355: F, t63359: F, t63361: F, t63366: F, t63369: F, t63371: F, t63374: F, t63377: F, t52035: F, t52037: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52065: F, t63393: F, t63396: F, t63399: F, t63469: F, t63471: F, t41330: F, t41332: F, t63474: F, t63476: F, t63478: F, t63480: F, t63482: F, t63485: F, t63488: F, t63491: F, t63494: F, t63497: F, t63500: F, t63503: F, t63505: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63673, t63676, t63679, t63681, t63683) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3391::<F>(t15386, t52508, t4732, t52452, t981, t2873, t6104, t2876, t15520, t4719, t19082, t3022);
        let (t63685, t63700) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3392::<F>(t19150, t3022, t51909, t51911, t51913, t51915, t51917, t51921, t51923, t63238, t63240, t63242, t63246, t63250, t63255, t63260);
        let t63715 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3393::<F>(t41246, t41281, t41285, t41287, t51937, t51942, t63266, t63268, t63274, t63276, t63278, t63281, t63285, t63290, t63293);
        let t63731 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3394::<F>(t41307, t41361, t41363, t51967, t51973, t51978, t63299, t63304, t63308, t63311, t63315, t63320, t63325, t63328, t63332);
        let t63747 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3395::<F>(t52033, t63336, t63338, t63340, t63342, t63346, t63351, t63355, t63359, t63361, t63366, t63369, t63371, t63374, t63377);
        let t63764 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3396::<F>(t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52065, t63393, t63396, t63399, t63469, t63471);
        let t63780 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3397::<F>(t41330, t41332, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491, t63494, t63497, t63500, t63503, t63505);
    (t63673, t63676, t63679, t63681, t63683, t63685, t63700, t63715, t63731, t63747, t63764, t63780)
}
