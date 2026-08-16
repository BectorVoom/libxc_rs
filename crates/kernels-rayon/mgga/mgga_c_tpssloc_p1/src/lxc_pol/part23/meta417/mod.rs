//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta417 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1236;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1237;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta417(t13012: f64, t20927: f64, t12984: f64, t12998: f64, t5544: f64, t686: f64, t20933: f64, t2563: f64, t20923: f64, t41011: f64, t118: f64, t20756: f64, t41170: f64, t794: f64, t20800: f64, t2576: f64, t21008: f64, t9573: f64, t20896: f64, t2697: f64, t13360: f64, t5624: f64, t1516: f64, t58844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68073, t68110, t68116, t68118, t68122) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1236(t13012, t20927, t12984, t12998, t5544, t686, t20933, t2563, t20923, t41011, t118, t20756, t41170, t794);
        let (t68131, t68148, t68195, t68197, t68199) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1237(t118, t20800, t2576, t794, t21008, t9573, t20896, t2697, t13360, t5624, t1516, t58844);
    (t68073, t68110, t68116, t68118, t68122, t68131, t68148, t68195, t68197, t68199)
}
