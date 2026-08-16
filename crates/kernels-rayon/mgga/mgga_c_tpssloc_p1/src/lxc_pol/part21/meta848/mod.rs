//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta848 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3070;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3071;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta848(t4745: f64, t51246: f64, t14838: f64, t15051: f64, t15054: f64, t15057: f64, t51249: f64, t4786: f64, t51402: f64, t14850: f64, t15061: f64, t15064: f64, t15068: f64, t51120: f64, t11185: f64, t18677: f64, t1098: f64, t18245: f64, t1119: f64, t18686: f64, t3308: f64, t3312: f64, t5983: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t63731, t63733, t63735, t63737, t63739, t63741, t63743) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3070(t4745, t51246, t14838, t15051, t15054, t15057, t51249, t4786, t51402, t14850, t15061, t15064);
        let (t63745, t63747, t63752, t63754, t63757) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3071(t15068, t51120, t11185, t18677, t1098, t18245, t1119, t18686, t3308, t3312, t5983, t3316);
    (t63731, t63733, t63735, t63737, t63739, t63741, t63743, t63745, t63747, t63752, t63754, t63757)
}
