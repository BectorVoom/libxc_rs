//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2510/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2510(t3566: f64, t3781: f64, t5330: f64, t3362: f64, t404: f64, t1222: f64, t13007: f64, t140: f64, t13028: f64, t3700: f64, t697: f64, t43813: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44951 = t3566 * t3781;
    let t44952 = t44951 * t5330;
    let t44958 = 1.0_f64 / t404 / t3362;
    let t44965 = t1222 * t140 * t13007;
    let t44972 = t1222 * t140 * t13028;
    let t44980 = t1222 * t697 * t3700;
    let t45000 = 0.18467901234567901234e0_f64 * t43813;
    (t44952, t44958, t44965, t44972, t44980, t45000)
}
