//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 242/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk242(t164: f64, t980: f64, t177: f64, t38: f64, t8: f64, t121: f64, t126: f64, t147: f64, t174: f64, t879: f64, t386: f64, t387: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t981 = t980 * t164;
    let t983 = 0.21437009059034868486e-3_f64 * t981 * t177;
    let t985 = 1.0_f64 / t8 / t38;
    let t986 = t121 * t985;
    let t987 = t986 * t126;
    let t989 = 35.0_f64 / 432.0_f64 * t987 * t147;
    let t991 = t174 * t879;
    let t993 = t386 * t387 * t991;
    (t983, t985, t986, t987, t989, t991, t993)
}
