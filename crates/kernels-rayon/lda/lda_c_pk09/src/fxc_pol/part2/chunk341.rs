//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 341/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk341(t1619: f64, t1652: f64, t1216: f64, t1517: f64, t1521: f64, t1522: f64, t1527: f64, t1529: f64, t1531: f64, t1532: f64, t1535: f64, t1538: f64, t1543: f64, t1546: f64, t1549: f64, t297: f64, t311: f64, t374: f64) -> (f64, f64) {
    let t1653 = t1619 + t1652;
    let t1655 = 5.40024514194619_f64 * t1517 + t1521 + 22.07984838129906_f64 * t1522 + t1527 - t1529 + t1531 - 2.427516195194328_f64 * t1532 * t311 - 2.2140749178833072_f64 * t1535 * t311 + 18.635258017632964_f64 * t1538 * t311 - 0.04115066352984959_f64 * t1216 * t374 + 19.489173774580152_f64 * t1543 * t311 + 4.937333717448355_f64 * t1546 * t311 + 1.8805371096875316_f64 * t1549 * t311 + t297 * t1653;
    (t1653, t1655)
}
