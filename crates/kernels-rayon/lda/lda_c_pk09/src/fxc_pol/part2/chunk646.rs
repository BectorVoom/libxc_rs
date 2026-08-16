//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 646/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk646(t1625: f64, t5584: f64, t1285: f64, t5308: f64, t1303: f64, t1336: f64, t360: f64, t4767: f64, t1382: f64, t1469: f64, t1475: f64, t1214: f64, t1471: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5585 = t5584 * t1625;
    let t5587 = t1285 * t5308;
    let t5589 = t1303 * t1336;
    let t5590 = t5589 * t1625;
    let t5593 = 8.661855010924512_f64 * t360 * t4767;
    let t5594 = t1382 * t1336;
    let t5595 = t5594 * t1625;
    let t5603 = t1469 * t1475;
    let t5604 = t1471 * t1214;
    (t5585, t5587, t5590, t5593, t5595, t5603, t5604)
}
