//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2281/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2281(t41282: f64, t4205: f64, t9926: f64, t1462: f64, t40709: f64, t13126: f64, t2250: f64, t4194: f64, t4195: f64, t9258: f64, t12890: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47149 = 36.0_f64 * t41282;
    let t47151 = 4.0_f64 * t4205 * t9926;
    let t47153 = 4.0_f64 * t40709 * t1462;
    let t47156 = 36.0_f64 * t4194 * t13126 * t2250;
    let t47159 = 12.0_f64 * t4194 * t4195 * t9258;
    let t47160 = t12890 * t751;
    (t47149, t47151, t47153, t47156, t47159, t47160)
}
