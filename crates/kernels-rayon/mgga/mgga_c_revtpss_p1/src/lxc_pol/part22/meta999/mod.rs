//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta999 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3391;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3392;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3393;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3394;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3395;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3396;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3397;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta999(t15386: f64, t52508: f64, t4732: f64, t52452: f64, t981: f64, t2873: f64, t6104: f64, t2876: f64, t15520: f64, t4719: f64, t19082: f64, t3022: f64, t19150: f64, t51909: f64, t51911: f64, t51913: f64, t51915: f64, t51917: f64, t51921: f64, t51923: f64, t63238: f64, t63240: f64, t63242: f64, t63246: f64, t63250: f64, t63255: f64, t63260: f64, t41246: f64, t41281: f64, t41285: f64, t41287: f64, t51937: f64, t51942: f64, t63266: f64, t63268: f64, t63274: f64, t63276: f64, t63278: f64, t63281: f64, t63285: f64, t63290: f64, t63293: f64, t41307: f64, t41361: f64, t41363: f64, t51967: f64, t51973: f64, t51978: f64, t63299: f64, t63304: f64, t63308: f64, t63311: f64, t63315: f64, t63320: f64, t63325: f64, t63328: f64, t63332: f64, t52033: f64, t63336: f64, t63338: f64, t63340: f64, t63342: f64, t63346: f64, t63351: f64, t63355: f64, t63359: f64, t63361: f64, t63366: f64, t63369: f64, t63371: f64, t63374: f64, t63377: f64, t52035: f64, t52037: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52065: f64, t63393: f64, t63396: f64, t63399: f64, t63469: f64, t63471: f64, t41330: f64, t41332: f64, t63474: f64, t63476: f64, t63478: f64, t63480: f64, t63482: f64, t63485: f64, t63488: f64, t63491: f64, t63494: f64, t63497: f64, t63500: f64, t63503: f64, t63505: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63673, t63676, t63679, t63681, t63683) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3391(t15386, t52508, t4732, t52452, t981, t2873, t6104, t2876, t15520, t4719, t19082, t3022);
        let (t63685, t63700) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3392(t19150, t3022, t51909, t51911, t51913, t51915, t51917, t51921, t51923, t63238, t63240, t63242, t63246, t63250, t63255, t63260);
        let t63715 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3393(t41246, t41281, t41285, t41287, t51937, t51942, t63266, t63268, t63274, t63276, t63278, t63281, t63285, t63290, t63293);
        let t63731 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3394(t41307, t41361, t41363, t51967, t51973, t51978, t63299, t63304, t63308, t63311, t63315, t63320, t63325, t63328, t63332);
        let t63747 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3395(t52033, t63336, t63338, t63340, t63342, t63346, t63351, t63355, t63359, t63361, t63366, t63369, t63371, t63374, t63377);
        let t63764 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3396(t52035, t52037, t52039, t52041, t52045, t52047, t52049, t52051, t52065, t63393, t63396, t63399, t63469, t63471);
        let t63780 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3397(t41330, t41332, t63474, t63476, t63478, t63480, t63482, t63485, t63488, t63491, t63494, t63497, t63500, t63503, t63505);
    (t63673, t63676, t63679, t63681, t63683, t63685, t63700, t63715, t63731, t63747, t63764, t63780)
}
