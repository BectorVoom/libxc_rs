//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1728;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1729;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1730;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta417(t18630: f64, t18673: f64, t18789: f64, t18906: f64, t300: f64, t3400: f64, t6084: f64, t4883: f64, t1164: f64, t18247: f64, t18249: f64, t18251: f64, t18257: f64, t18261: f64, t18264: f64, t18268: f64, t18270: f64, t18273: f64, t18278: f64, t18282: f64, t18285: f64, t18672: f64, t18676: f64, t18679: f64, t6063: f64, t1166: f64, t4858: f64, t4874: f64, t3411: f64, t6098: f64, t4869: f64, t4884: f64, t1147: f64, t1156: f64, t18785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18909, t18910, t18911, t18913, t18914) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1728(t18630, t18673, t18789, t18906, t300, t3400, t6084, t4883, t1164, t18247, t18249, t18251, t18257, t18261, t18264, t18268, t18270, t18273, t18278, t18282, t18285, t18672, t18676, t18679);
        let t18915 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1729(t300, t6063);
        let (t18917, t18918, t18920, t18922, t18924, t18926) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1730(t1166, t18915, t4858, t4874, t1164, t3411, t6098, t4869, t4884, t1147, t1156, t18785);
    (t18909, t18910, t18911, t18913, t18914, t18915, t18917, t18918, t18920, t18922, t18924, t18926)
}
