//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 409/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk409(t1971: f64, t451: f64, t1754: f64, t1765: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64, t447: f64, t452: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2091 = t451 * t1971;
    let t2094 = 1.4770435158815312_f64 * t1754;
    let t2096 = 0.49234783862717707_f64 * t1765;
    let t2098 = 0.2946275542389858_f64 * t1684;
    let t2100 = 0.0982091847463286_f64 * t1735;
    let t2102 = t2094 - 1.4770435158815312_f64 * t1762 + t2096 + 1.4770435158815312_f64 * t1769 + t2098 - 0.2946275542389858_f64 * t1732 + t2100 + 0.2946275542389858_f64 * t1738;
    let t2103 = t447 * t2102;
    let t2104 = t2103 * t452;
    (t2091, t2094, t2096, t2098, t2100, t2102, t2103, t2104)
}
