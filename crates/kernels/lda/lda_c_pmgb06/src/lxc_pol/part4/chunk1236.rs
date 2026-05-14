//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1236/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1236<F: Float>(t10500: F, t10505: F, t10509: F, t10511: F, t10515: F, t10518: F, t10520: F, t10522: F, t10525: F, t10528: F, t10531: F, t10533: F, t18436: F, t10494: F, t1338: F, t18418: F, t18420: F, t18422: F, t18424: F, t18426: F, t18428: F, t18430: F, t18432: F, t18434: F, t2422: F, t399: F, t4435: F, t6939: F, t795: F, t84: F) -> (F, F) {
    let t18437 = t10500 + t10505 + t10509 - t10511 + t10515 - t10518 - t10520 + t10522 - t18436 + t10525 + t10528 - t10531 + t10533;
    let t18440 = 1.0051538464260528 * t10494 - 0.1675256410710088 * t795 * t4435 - 0.0837628205355044 * t1338 * t2422 - 0.1675256410710088 * t399 * t6939 - 0.3350512821420176 * t18418 - 0.1675256410710088 * t18420 - 0.1675256410710088 * t18422 + 0.3350512821420176 * t18424 + 0.1675256410710088 * t18426 + 0.1675256410710088 * t18428 + 0.1675256410710088 * t18430 + 0.1675256410710088 * t18432 + 0.3350512821420176 * t18434 - 0.0837628205355044 * t84 * t18437;
    (t18437, t18440)
}
