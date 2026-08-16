//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta347 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1745;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1746;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1747;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta347(t221: f64, t2379: f64, t4128: f64, t1489: f64, t9541: f64, t4126: f64, t782: f64, t4130: f64, t12971: f64, t210: f64, t214: f64, t2563: f64, t4138: f64, t4134: f64, t9546: f64, t118: f64, t4119: f64, t794: f64, t2576: f64, t13005: f64, t787: f64, t9572: f64, t9574: f64, t9579: f64, t9583: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13007, t13010, t13012) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1745(t221, t2379, t4128, t1489, t9541, t4126, t782);
        let (t13014, t13017, t13020, t13022, t13025) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1746(t13012, t4130, t12971, t210, t214, t2563, t4138, t4134, t9546, t118, t4119, t794);
        let (t13027, t13028) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1747(t13025, t2576, t13005, t13007, t13010, t13014, t13017, t13020, t13022, t787, t9572, t9574, t9579, t9583);
    (t13007, t13010, t13012, t13014, t13017, t13020, t13022, t13025, t13027, t13028)
}
