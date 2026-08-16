//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 317/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk317(t1435: f64, t404: f64, t378: f64, t1263: f64, t1272: f64, t1268: f64, t1275: f64, t305: f64, t1284: f64, t334: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1437 = 0.8091720650647759_f64 * t404 * t1435;
    let t1438 = t378 * t378;
    let t1439 = 1.0_f64 / t1438;
    let t1440 = 1.5625_f64 * t1263;
    let t1442 = 0.5208333333333334_f64 * t1272;
    let t1444 = t1440 - 1.5625_f64 * t1268 + t1442 + 1.5625_f64 * t1275;
    let t1445 = t1439 * t1444;
    let t1447 = 0.025613155472356368_f64 * t1439 + 1.0_f64;
    let t1448 = 1.0_f64 / t1447;
    let t1449 = t1448 * t305;
    let t1450 = t1445 * t1449;
    let t1451 = t1284 * t334;
    (t1437, t1438, t1439, t1440, t1442, t1444, t1445, t1447, t1448, t1449, t1450, t1451)
}
