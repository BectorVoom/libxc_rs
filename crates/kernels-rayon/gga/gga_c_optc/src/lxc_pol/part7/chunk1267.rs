//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1267/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1267(t2994: f64, t1056: f64, t8582: f64, t3012: f64, t2993: f64, t3018: f64, t3020: f64, t2917: f64) -> (f64, f64, f64, f64, f64) {
    let t26153 = t2994 * t2994;
    let t26156 = 24.0_f64 * t8582 * t26153 * t1056;
    let t26157 = t3012 * t3012;
    let t26160 = 6.0_f64 * t2993 * t26157 * t1056;
    let t26163 = 0.48245472966453314466e2_f64 * t3018 * t26157 * t3020;
    let t26164 = t2917 * t2917;
    (t26153, t26156, t26160, t26163, t26164)
}
