//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta752 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2525;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2526;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2527;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2528;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta752(t47774: f64, t50998: f64, t68513: f64, t43816: f64, t44348: f64, t51565: f64, t51574: f64, t63361: f64, t63382: f64, t63384: f64, t63398: f64, t63400: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t71183: f64, t71187: f64, t71191: f64, t71195: f64, t71199: f64, t71203: f64, t423: f64, t71162: f64, t1157: f64, t1164: f64, t21938: f64, t3375: f64, t1254: f64, t19270: f64, t4700: f64, t5091: f64, t71095: f64, t71097: f64, t71101: f64, t71106: f64, t71109: f64, t71112: f64, t71114: f64, t71118: f64, t4861: f64, t64525: f64, t21833: f64, t3411: f64, t18786: f64, t4874: f64, t21826: f64, t300: f64, t1166: f64, t22236: f64, t4883: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t71206 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2525(t47774, t50998, t68513);
        let t71214 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2526(t43816, t44348, t51565, t51574, t63361, t63382, t63384, t63398, t63400, t71166, t71170, t71174, t71179, t71183, t71187, t71191, t71195, t71199, t71203, t71206);
        let (t71217, t71221, t71222) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2527(t423, t71162, t71214, t1157, t1164, t21938, t3375, t1254, t19270, t4700, t5091, t71095, t71097, t71101, t71106, t71109, t71112, t71114, t71118);
        let (t71225, t71227, t71230, t71233, t71236) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2528(t1164, t4861, t64525, t21833, t3411, t18786, t4874, t21826, t300, t1166, t22236, t4883);
    (t71206, t71217, t71221, t71222, t71225, t71227, t71230, t71233, t71236)
}
