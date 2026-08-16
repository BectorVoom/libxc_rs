//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta290 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1597;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1598;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1599;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1600;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta290(t10294: f64, t268: f64, t271: f64, t6546: f64, t2394: f64, t885: f64, t2772: f64, t690: f64, t2777: f64, t2781: f64, t154: f64, t3061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10542, t10544, t10545, t10556) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1597(t10294, t268, t271, t6546, t2394, t885);
        let t10558 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1598(t2772, t690);
        let t10560 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1599(t2777, t690);
        let t10562 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1600(t2781, t690);
        let t10564 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1601(t154, t3061);
    (t10542, t10544, t10545, t10556, t10558, t10560, t10562, t10564)
}
