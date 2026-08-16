//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1106/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1106(t4147: f64, t8107: f64, t8713: f64, t1450: f64, t211: f64, t9644: f64, t675: f64, t886: f64, t11006: f64, t256: f64, t2410: f64, t10308: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t34495 = t4147 * t8107;
    let t37318 = t4147 * t8713;
    let t38099 = t8713 * t1450;
    let t39643 = 1.0_f64 / t9644 / t211;
    let t41040 = t675 * t886;
    let t41077 = 1.0_f64 / t11006 / t256;
    let t41153 = t2410 * t2410;
    let t41154 = 1.0_f64 / t41153;
    let t45963 = t599 * t10308;
    (t34495, t37318, t38099, t39643, t41040, t41077, t41154, t45963)
}
