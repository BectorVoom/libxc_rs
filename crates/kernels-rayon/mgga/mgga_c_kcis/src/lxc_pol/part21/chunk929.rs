//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 929/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk929(t1021: f64, t14106: f64, t1092: f64, t1773: f64, t3190: f64, t3218: f64, t10338: f64, t1754: f64, t2943: f64, t304: f64, t2944: f64, t4601: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14107 = t1021 * t14106;
    let t14108 = t1092 * t14107;
    let t14110 = t1773 * t3190;
    let t14111 = t3218 * t14110;
    let t14112 = t1021 * t14111;
    let t14113 = t1092 * t14112;
    let t14115 = t10338 * t1754;
    let t14117 = t304 * t2943;
    let t14118 = t4601 * t2944;
    (t14108, t14110, t14113, t14115, t14117, t14118)
}
