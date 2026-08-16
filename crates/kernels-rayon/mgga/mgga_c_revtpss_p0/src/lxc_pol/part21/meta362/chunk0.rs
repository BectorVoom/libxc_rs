//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1722/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1722(t1021: f64, t3201: f64, t362: f64, t40: f64, t361: f64, t351: f64, t1058: f64, t3231: f64, t1054: f64, t2434: f64, t371: f64, t373: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11956 = t1021 * t3201;
    let t11958 = t362 * t362;
    let t11960 = 1.0_f64 / t40 / t11958;
    let t11961 = t361 * t11960;
    let t11962 = t351 * t11961;
    let t11965 = t3231 * t1058;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    (t11956, t11960, t11961, t11962, t11965, t11967, t11970)
}
