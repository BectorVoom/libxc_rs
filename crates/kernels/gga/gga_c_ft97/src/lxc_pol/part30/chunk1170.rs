//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1170/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1170<F: Float>(t2842: F, t7611: F, t312: F, t35972: F, t10447: F, t10683: F, t10703: F, t1091: F, t11593: F, t143989: F, t144140: F, t144142: F, t144148: F, t144150: F, t144153: F, t1495: F, t1508: F, t152872: F, t153473: F, t15369: F, t15460: F, t1901: F, t193: F, t25253: F, t28501: F, t28506: F, t2862: F, t2874: F, t2881: F, t29127: F, t29130: F, t295: F, t319: F, t34118: F, t34197: F, t35828: F, t35863: F, t36156: F, t36182: F, t3746: F, t4176: F, t4181: F, t44369: F, t446: F, t684: F, t7045: F, t840: F, t882: F, t89: F) -> F {
    let t154705 = t2842 * t7611;
    let t154717 = t312 * t35972;
    let t154726 = -F::new(4.0) * t1901 * t29127 * t1495 * t29130 + F::new(2.0) / F::new(27.0) * t144140 - F::new(4.0) / F::new(9.0) * t144142 + F::new(2.0) / F::new(3.0) * t446 * t2862 * t882 * t35863 + F::new(2.0) / F::new(3.0) * t446 * t2862 * t319 * t152872 + F::new(4.0) / F::new(3.0) * t446 * t2862 * t1508 * t28501 + F::new(4.0) / F::new(3.0) * t446 * t2862 * t1508 * t28506 - F::new(2.0) * t446 * t10683 * t882 * t35828 - F::new(2.0) / F::new(9.0) * t1901 * t44369 * t36182 - F::new(2.0) / F::new(9.0) * t1901 * t10703 * t34118 * t1091 + F::new(2.0) / F::new(3.0) * t144148 + F::new(2.0) / F::new(9.0) * t144150 - F::new(4.0) / F::new(9.0) * t144153 + t89 * t193 * t295 * t153473 * t312 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1901 * t15369 * t143989 * t4176 - F::new(2.0) / F::new(3.0) * t1901 * t15460 * t154705 * t4181 - F::new(2.0) / F::new(9.0) * t1901 * t10447 * t36156 + F::new(4.0) / F::new(9.0) * t11593 * t2881 * t34197 * t3746 + t1901 * t2874 * t154717 * t684 / F::new(9.0) + F::new(2.0) / F::new(3.0) * t446 * t840 * t25253 * t7045;
    t154726
}
