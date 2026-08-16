//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 569/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk569(t142: f64, t1557: f64, t1570: f64, t548: f64, t135: f64, t8078: f64, t40: f64, t6: f64, t12: f64, t171: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8766 = t142 * t1557;
    let t8774 = t142 * t1570;
    let t8906 = t548 * t548;
    let t8907 = 1.0_f64 / t8906;
    let t8908 = t135 * t8907;
    let t8914 = 0.18521666970164609055e-1_f64 * t8078;
    let t8946 = t6 / t40;
    let t8947 = t12 * t171;
    let t8948 = t8946 * t8947;
    (t8766, t8774, t8906, t8907, t8908, t8914, t8946, t8947, t8948)
}
