//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 941/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk941(t17383: f64, t8611: f64, t11671: f64, t11677: f64, t14881: f64, t14883: f64, t14885: f64, t14887: f64, t14889: f64, t14895: f64, t17381: f64, t17384: f64, t17389: f64, t17392: f64, t8640: f64) -> (f64, f64) {
    let t17394 = t8611 * t17383;
    let t17396 = -0.32862666666666666666e0_f64 * t14881 + 0.16431333333333333333e0_f64 * t14883 + 0.19931111111111111111e0_f64 * t14885 - 0.59793333333333333333e0_f64 * t14887 + 0.29896666666666666667e0_f64 * t14889 + 0.5477111111111111111e-1_f64 * t14895 + 0.1898925e1_f64 * t17381 + 0.142419375e1_f64 * t17384 - 0.39862222222222222223e0_f64 * t11671 - 0.27385555555555555556e0_f64 * t11677 - 0.82156666666666666668e-1_f64 * t17389 + 0.49293999999999999999e0_f64 * t17392 - 0.76790625e-1_f64 * t17394 - t8640;
    (t17394, t17396)
}
