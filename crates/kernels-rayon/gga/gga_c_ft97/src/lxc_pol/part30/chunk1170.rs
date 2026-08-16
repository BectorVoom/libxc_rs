//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1170/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1170(t2842: f64, t7611: f64, t312: f64, t35972: f64, t10447: f64, t10683: f64, t10703: f64, t1091: f64, t11593: f64, t143989: f64, t144140: f64, t144142: f64, t144148: f64, t144150: f64, t144153: f64, t1495: f64, t1508: f64, t152872: f64, t153473: f64, t15369: f64, t15460: f64, t1901: f64, t193: f64, t25253: f64, t28501: f64, t28506: f64, t2862: f64, t2874: f64, t2881: f64, t29127: f64, t29130: f64, t295: f64, t319: f64, t34118: f64, t34197: f64, t35828: f64, t35863: f64, t36156: f64, t36182: f64, t3746: f64, t4176: f64, t4181: f64, t44369: f64, t446: f64, t684: f64, t7045: f64, t840: f64, t882: f64, t89: f64) -> f64 {
    let t154705 = t2842 * t7611;
    let t154717 = t312 * t35972;
    let t154726 = -4.0_f64 * t1901 * t29127 * t1495 * t29130 + 2.0_f64 / 27.0_f64 * t144140 - 4.0_f64 / 9.0_f64 * t144142 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t882 * t35863 + 2.0_f64 / 3.0_f64 * t446 * t2862 * t319 * t152872 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t1508 * t28501 + 4.0_f64 / 3.0_f64 * t446 * t2862 * t1508 * t28506 - 2.0_f64 * t446 * t10683 * t882 * t35828 - 2.0_f64 / 9.0_f64 * t1901 * t44369 * t36182 - 2.0_f64 / 9.0_f64 * t1901 * t10703 * t34118 * t1091 + 2.0_f64 / 3.0_f64 * t144148 + 2.0_f64 / 9.0_f64 * t144150 - 4.0_f64 / 9.0_f64 * t144153 + t89 * t193 * t295 * t153473 * t312 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1901 * t15369 * t143989 * t4176 - 2.0_f64 / 3.0_f64 * t1901 * t15460 * t154705 * t4181 - 2.0_f64 / 9.0_f64 * t1901 * t10447 * t36156 + 4.0_f64 / 9.0_f64 * t11593 * t2881 * t34197 * t3746 + t1901 * t2874 * t154717 * t684 / 9.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t840 * t25253 * t7045;
    t154726
}
