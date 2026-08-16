//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 332/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk332(t1214: f64, t395: f64, t382: f64, t359: f64, t365: f64, t355: f64, t1243: f64, t1255: f64, t1263: f64, t1272: f64, t1251: f64, t1259: f64, t1268: f64, t1275: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1535 = t395 * t1214;
    let t1538 = t382 * t1214;
    let t1543 = t359 * t1214;
    let t1546 = t365 * t1214;
    let t1549 = t355 * t1214;
    let t1552 = 1.4770435158815312_f64 * t1243;
    let t1554 = 0.49234783862717707_f64 * t1255;
    let t1556 = 0.2946275542389858_f64 * t1263;
    let t1558 = 0.0982091847463286_f64 * t1272;
    let t1560 = t1552 - 1.4770435158815312_f64 * t1251 + t1554 + 1.4770435158815312_f64 * t1259 + t1556 - 0.2946275542389858_f64 * t1268 + t1558 + 0.2946275542389858_f64 * t1275;
    (t1535, t1538, t1543, t1546, t1549, t1552, t1554, t1556, t1558, t1560)
}
