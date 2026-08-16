//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 454/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk454(t2447: f64, t271: f64, t1181: f64, t1184: f64, t1703: f64, t1704: f64, t1180: f64, t1153: f64, t1164: f64, t253: f64) -> (f64, f64, f64, f64) {
    let t2448 = t2447 * t271;
    let t2451 = -t1181 + t1703 + t1704 - t1184;
    let t2452 = t1180 * t2451;
    let t2455 = t1153 - t1164 + 1.28_f64 * t253 * t2448 - 1.28_f64 * t253 * t2452;
    (t2448, t2451, t2452, t2455)
}
