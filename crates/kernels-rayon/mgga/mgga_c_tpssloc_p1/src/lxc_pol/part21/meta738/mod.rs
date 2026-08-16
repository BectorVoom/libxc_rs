//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2600;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2601;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta738(t3577: f64, t44951: f64, t4953: f64, t11677: f64, t15245: f64, t1174: f64, t14753: f64, t3431: f64, t14744: f64, t11651: f64, t15438: f64, t1227: f64, t13969: f64, t15540: f64, t15530: f64, t3515: f64, t11702: f64, t5002: f64, t11708: f64, t15502: f64, t15506: f64, t15554: f64, t3506: f64, t10469: f64, t1720: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52758, t52766, t52773, t52776, t52781, t52792) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2600(t3577, t44951, t4953, t11677, t15245, t1174, t14753, t3431, t14744, t11651, t15438, t1227, t13969, t15540);
        let (t52795, t52801, t52810, t52813, t52817, t52834) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2601(t13969, t15530, t3515, t11702, t5002, t11708, t15502, t15506, t15554, t3506, t10469, t1720);
    (t52758, t52766, t52773, t52776, t52781, t52792, t52795, t52801, t52810, t52813, t52817, t52834)
}
