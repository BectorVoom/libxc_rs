//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 919/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk919<F: Float>(t11351: F, t337: F, t430: F, t11059: F, t489: F, t2738: F, t6247: F, t6977: F, t2739: F, t7473: F, t2042: F, t545: F, t11179: F, t455: F, t552: F, t6739: F, t6740: F, t6743: F, t6764: F, t6771: F, t6792: F, t6793: F, t6804: F, t6806: F, t6811: F, t6816: F, t6823: F, t6827: F) -> (F, F) {
    let t11352 = t11351 * t337;
    let t11353 = t11352 * t430;
    let t11356 = t489 * t11059;
    let t11362 = t6247 * t2738;
    let t11363 = t11362 * t6977;
    let t11366 = t2739 * t7473;
    let t11367 = t11366 * t2042;
    let t11369 = t545 * t11059;
    let t11375 = -t6739 + 6.496391258193384 * t6740 - 6.496391258193384 * t6743 - t6764 - t6771 - 1.8805371096875316 * t11353 * t552 - 3.7610742193750633 * t11356 * t455 + t6792 - 7.35994946043302 * t6793 + t6804 - 3.600163427964126 * t6806 + 22.07984838129906 * t6811 + 5.9648145914819635 * t11363 * t11179 + 2.9824072957409817 * t11367 - 2.427516195194328 * t11369 * t455 - 10.80049028389238 * t6816 - 22.07984838129906 * t6823 + 10.80049028389238 * t6827;
    (t11352, t11375)
}
