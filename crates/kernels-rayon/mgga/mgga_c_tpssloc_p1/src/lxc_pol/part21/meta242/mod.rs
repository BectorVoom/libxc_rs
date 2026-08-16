//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta242 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1428;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1429;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta242(t1088: f64, t5979: f64, t123: f64, t3237: f64, t4721: f64, t5973: f64, t5977: f64, t423: f64, t1671: f64, t4740: f64, t1670: f64, t1118: f64, t3264: f64, t1661: f64, t3270: f64, t3274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t5980, t5981, t5983, t5985, t5987, t5988, t5989) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1428(t1088, t5979, t123, t3237, t4721, t5973, t5977, t423, t1671, t4740, t1670, t1118);
        let (t5991, t5992) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1429(t3264, t5989, t1661);
        let (t5993, t5999) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1430(t3270, t5992, t3274, t4721, t5973, t5977, t5981);
    (t5980, t5981, t5983, t5985, t5987, t5988, t5989, t5991, t5992, t5993, t5999)
}
