//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1474/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1474(t2874: f64, t41510: f64, t935: f64, t2866: f64, t2873: f64, t2876: f64, t11298: f64, t910: f64, t11301: f64, t11385: f64, t2926: f64, t41500: f64) -> (f64, f64, f64, f64) {
    let t41879 = 6.0_f64 * t2874 * t41510 * t935;
    let t41880 = t2866 * t2873;
    let t41882 = 12.0_f64 * t41880 * t2876;
    let t41883 = t910 * t11298;
    let t41885 = 0.3859675079686208416e3_f64 * t41883 * t11301;
    let t41888 = 0.57895126195293126241e3_f64 * t11385 * t41500 * t2926;
    (t41879, t41882, t41885, t41888)
}
