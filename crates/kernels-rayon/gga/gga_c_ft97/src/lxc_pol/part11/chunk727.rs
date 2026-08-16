//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 727/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk727(t9768: f64, t9765: f64, t2475: f64, t747: f64, t2514: f64, t91: f64, t251: f64, t631: f64, t675: f64, t7242: f64, t898: f64, t2476: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9872 = 2.0_f64 / 9.0_f64 * t9768;
    let t9876 = 2.0_f64 / 9.0_f64 * t9765;
    let t9881 = t2475 * t747;
    let t9883 = t91 * t9881 * t2514;
    let t9890 = 1.0_f64 / t251 / t631 / t898 / t675 / t7242 / 4.0_f64;
    let t9891 = t2476 * t747;
    (t9872, t9876, t9881, t9883, t9890, t9891)
}
