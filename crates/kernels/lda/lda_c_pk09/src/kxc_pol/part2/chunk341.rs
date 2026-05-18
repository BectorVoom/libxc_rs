//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 341/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk341<F: Float>(t1619: F, t1652: F, t1216: F, t1517: F, t1521: F, t1522: F, t1527: F, t1529: F, t1531: F, t1532: F, t1535: F, t1538: F, t1543: F, t1546: F, t1549: F, t297: F, t311: F, t374: F) -> (F, F) {
    let t1653 = t1619 + t1652;
    let t1655 = F::new(5.40024514194619) * t1517 + t1521 + F::new(22.07984838129906) * t1522 + t1527 - t1529 + t1531 - F::new(2.427516195194328) * t1532 * t311 - F::new(2.2140749178833072) * t1535 * t311 + F::new(18.635258017632964) * t1538 * t311 - F::new(0.04115066352984959) * t1216 * t374 + F::new(19.489173774580152) * t1543 * t311 + F::new(4.937333717448355) * t1546 * t311 + F::new(1.8805371096875316) * t1549 * t311 + t297 * t1653;
    (t1653, t1655)
}
