//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1199/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1199(t10560: f64, t1154: f64, t6272: f64, t3405: f64, t6276: f64, t1155: f64, t18443: f64, t3393: f64, t6673: f64, t1045: f64, t1727: f64, t14215: f64) -> (f64, f64, f64, f64, f64) {
    let t20020 = t1154 * t10560 * t6272;
    let t20024 = t1154 * t3405 * t6276;
    let t20028 = t1154 * t1155 * t18443;
    let t20031 = t3393 * t6673;
    let t20033 = t1727 * t1045;
    let t20034 = t14215 * t20033;
    (t20020, t20024, t20028, t20031, t20034)
}
