//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1952;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1953;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1954;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1955;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta505(t1088: f64, t21769: f64, t123: f64, t21749: f64, t1089: f64, t20217: f64, t11247: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64, t21760: f64, t21764: f64, t21767: f64, t1107: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21770, t21771) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1951(t1088, t21769, t123);
        let (t21773, t21774) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1952(t1088, t21749, t123);
        let t21776 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1953(t1089, t20217);
        let (t21777, t21778) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1954(t1088, t21776, t123);
        let (t21780, t21781) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1955(t11247, t14702, t18203, t18219, t18229, t21760, t21764, t21767, t21771, t21774, t21778, t1107);
    (t21770, t21771, t21773, t21774, t21776, t21777, t21778, t21780, t21781)
}
