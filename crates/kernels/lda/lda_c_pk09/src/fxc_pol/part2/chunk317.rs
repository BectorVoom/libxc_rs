//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 317/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk317<F: Float>(t1435: F, t404: F, t378: F, t1263: F, t1272: F, t1268: F, t1275: F, t305: F, t1284: F, t334: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1437 = F::cast_from(0.8091720650647759_f64) * t404 * t1435;
    let t1438 = t378 * t378;
    let t1439 = F::cast_from(1.0_f64) / t1438;
    let t1440 = F::cast_from(1.5625_f64) * t1263;
    let t1442 = F::cast_from(0.5208333333333334_f64) * t1272;
    let t1444 = t1440 - F::cast_from(1.5625_f64) * t1268 + t1442 + F::cast_from(1.5625_f64) * t1275;
    let t1445 = t1439 * t1444;
    let t1447 = F::cast_from(0.025613155472356368_f64) * t1439 + F::cast_from(1.0_f64);
    let t1448 = F::cast_from(1.0_f64) / t1447;
    let t1449 = t1448 * t305;
    let t1450 = t1445 * t1449;
    let t1451 = t1284 * t334;
    (t1437, t1438, t1439, t1440, t1442, t1444, t1445, t1447, t1448, t1449, t1450, t1451)
}
