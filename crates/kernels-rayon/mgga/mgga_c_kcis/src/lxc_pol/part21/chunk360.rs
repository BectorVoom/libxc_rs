//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 360/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk360(t1142: f64, t1820: f64, t1205: f64, t1664: f64, t1214: f64, t1217: f64, t1671: f64, t1674: f64, t1677: f64, t1220: f64) -> (f64, f64, f64, f64) {
    let t1821 = t1142 * t1820;
    let t1823 = -t1205 - 0.17123333333333333333e-1_f64 * t1664;
    let t1830 = 0.3529725e1_f64 * t1671 - t1214 - 0.516475e0_f64 * t1664 + 0.6311625e0_f64 * t1674 - t1217 - 0.104195e0_f64 * t1677;
    let t1831 = t1830 * t1220;
    (t1821, t1823, t1830, t1831)
}
