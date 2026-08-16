//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2495/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2495(t1209: f64, t13147: f64, t17708: f64, t11249: f64, t13043: f64, t12804: f64, t12916: f64, t3718: f64, t12854: f64, t17350: f64, t12808: f64, t12865: f64, t12909: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44500 = t1209 * t13147 * t17708;
    let t44501 = t13043 * t11249;
    let t44508 = t3718 * t12916 * t12804;
    let t44510 = t12854 * t17350;
    let t44517 = t12808 * t17350;
    let t44521 = t12909 * t12865;
    (t44500, t44501, t44508, t44510, t44517, t44521)
}
