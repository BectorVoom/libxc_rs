//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta860 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3119;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3120;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3121;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta860(t1164: f64, t15133: f64, t4874: f64, t11433: f64, t18910: f64, t1695: f64, t51810: f64, t64482: f64, t11126: f64, t6098: f64, t6102: f64, t18785: f64, t3400: f64, t4883: f64, t15044: f64, t4869: f64, t3378: f64, t63446: f64, t63449: f64, t63451: f64, t63557: f64, t63560: f64, t63563: f64, t14842: f64, t11292: f64, t6084: f64, t3404: f64, t3637: f64, t43706: f64, t4700: f64, t6274: f64, t63566: f64, t63568: f64, t63571: f64, t63574: f64, t63576: f64, t63579: f64, t63582: f64, t63585: f64, t63587: f64, t63591: f64, t63594: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t64514, t64517, t64520, t64522, t64524, t64525) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3119(t1164, t15133, t4874, t11433, t18910, t1695, t51810, t64482, t11126, t6098, t6102, t18785, t3400);
        let (t64528, t64530, t64533, t64534) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3120(t1164, t4883, t64525, t15044, t4869, t18910, t3378, t63446, t63449, t63451, t63557, t63560, t63563, t64514, t64517, t64520, t64522, t64524);
        let (t64536, t64540, t64545) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3121(t14842, t4869, t11292, t6084, t1164, t3404, t3637, t43706, t4700, t6274, t63566, t63568, t63571, t63574, t63576, t63579, t63582, t63585, t63587, t63591, t63594);
    (t64514, t64517, t64520, t64522, t64524, t64528, t64530, t64533, t64534, t64536, t64540, t64545)
}
