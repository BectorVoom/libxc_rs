//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1158/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1158(t10262: f64, t10683: f64, t10703: f64, t10726: f64, t10758: f64, t1901: f64, t2405: f64, t2413: f64, t2857: f64, t2894: f64, t319: f64, t41753: f64, t41757: f64, t44426: f64, t44428: f64, t44436: f64, t44445: f64, t446: f64, t684: f64, t835: f64, t871: f64, t875: f64, t882: f64, t9572: f64, t9596: f64) -> f64 {
    let t44467 = 8.0_f64 / 3.0_f64 * t44426 + 16.0_f64 / 9.0_f64 * t44428 - 4.0_f64 / 3.0_f64 * t1901 * t10703 * t10726 * t684 + t44436 - 40.0_f64 / 81.0_f64 * t446 * t10758 * t882 * t9572 - t446 * t835 * t319 * t41757 / 9.0_f64 - 80.0_f64 / 243.0_f64 * t446 * t44445 * t319 * t41753 - 4.0_f64 / 9.0_f64 * t446 * t835 * t882 * t9596 - 4.0_f64 / 9.0_f64 * t446 * t2857 * t2894 * t2405 - 2.0_f64 / 3.0_f64 * t446 * t835 * t2894 * t2413 + 8.0_f64 * t446 * t10683 * t871 * t10262 * t875;
    t44467
}
