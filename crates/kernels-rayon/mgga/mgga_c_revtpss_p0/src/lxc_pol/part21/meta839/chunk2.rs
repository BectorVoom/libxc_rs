//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3148/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3148(t16857: f64, t3399: f64, t12322: f64, t5071: f64, t1134: f64, t16926: f64, t3407: f64, t56159: f64, t56163: f64, t56167: f64, t58029: f64, t58032: f64, t58035: f64, t58038: f64, t58041: f64, t58044: f64) -> (f64, f64, f64, f64) {
    let t58046 = t16857 * t3399;
    let t58048 = t5071 * t12322;
    let t58051 = t3407 * t16926 * t1134;
    let t58053 = 0.929655e1_f64 * t56159 + 0.103295e1_f64 * t56163 + 0.123954e2_f64 * t56167 + 0.187551e1_f64 * t58029 + 0.13892666666666666667e0_f64 * t58032 - 0.62517e0_f64 * t58035 + 0.794188125e1_f64 * t58038 - 0.473371875e0_f64 * t58041 - 0.52945875e1_f64 * t58044 - 0.52945875e1_f64 * t58046 - 0.17648625e1_f64 * t58048 + 0.94674375e0_f64 * t58051;
    (t58046, t58048, t58051, t58053)
}
