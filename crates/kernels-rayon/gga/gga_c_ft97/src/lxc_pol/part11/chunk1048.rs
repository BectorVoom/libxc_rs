//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1048/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1048(t10024: f64, t41454: f64, t446: f64, t683: f64, t7514: f64, t505: f64, t668: f64, t9708: f64, t1882: f64, t9776: f64, t2409: f64, t2459: f64) -> (f64, f64, f64, f64, f64) {
    let t41823 = t446 * t10024 * t41454;
    let t41825 = t683 * t7514;
    let t41827 = t9708 * t668 * t505;
    let t41829 = t446 * t41825 * t41827;
    let t41831 = t1882 * t9776;
    let t41833 = t2409 * t2459;
    (t41823, t41827, t41829, t41831, t41833)
}
