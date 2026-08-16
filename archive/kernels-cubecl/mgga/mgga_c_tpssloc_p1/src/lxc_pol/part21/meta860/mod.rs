//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta860 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3119;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3120;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta860<F: Float>(t1164: F, t15133: F, t4874: F, t11433: F, t18910: F, t1695: F, t51810: F, t64482: F, t11126: F, t6098: F, t6102: F, t18785: F, t3400: F, t4883: F, t15044: F, t4869: F, t3378: F, t63446: F, t63449: F, t63451: F, t63557: F, t63560: F, t63563: F, t14842: F, t11292: F, t6084: F, t3404: F, t3637: F, t43706: F, t4700: F, t6274: F, t63566: F, t63568: F, t63571: F, t63574: F, t63576: F, t63579: F, t63582: F, t63585: F, t63587: F, t63591: F, t63594: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t64514, t64517, t64520, t64522, t64524, t64525) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3119::<F>(t1164, t15133, t4874, t11433, t18910, t1695, t51810, t64482, t11126, t6098, t6102, t18785, t3400);
        let (t64528, t64530, t64533, t64534) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3120::<F>(t1164, t4883, t64525, t15044, t4869, t18910, t3378, t63446, t63449, t63451, t63557, t63560, t63563, t64514, t64517, t64520, t64522, t64524);
        let (t64536, t64540, t64545) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3121::<F>(t14842, t4869, t11292, t6084, t1164, t3404, t3637, t43706, t4700, t6274, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587, t63591, t63594);
    (t64514, t64517, t64520, t64522, t64524, t64528, t64530, t64533, t64534, t64536, t64540, t64545)
}
