//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2416/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2416(t1214: f64, t13045: f64, t12854: f64, t17350: f64, t12808: f64, t12865: f64, t12909: f64, t13051: f64, t44173: f64, t13037: f64, t472: f64, t3603: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44502 = t13045 * t1214;
    let t44510 = t12854 * t17350;
    let t44517 = t12808 * t17350;
    let t44521 = t12909 * t12865;
    let t44526 = t44173 * t13051;
    let t44531 = 1.0_f64 / t13037 / t472;
    let t44535 = t3603 * t3603;
    (t44502, t44510, t44517, t44521, t44526, t44531, t44535)
}
