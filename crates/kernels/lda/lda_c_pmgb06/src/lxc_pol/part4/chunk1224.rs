//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1224/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1224<F: Float>(t17465: F, t17469: F, t17472: F, t17475: F, t17478: F, t17482: F, t17487: F, t17490: F, t17493: F, t17496: F, t17497: F, t17499: F, t17503: F, t17505: F, t17507: F, t17509: F, t17511: F, t17513: F, t17515: F, t17517: F, t17527: F, t17530: F, t17532: F, t17534: F, t17537: F, t17542: F, t17553: F, t17555: F, t17557: F, t17559: F) -> (F, F) {
    let t18308 = -t17465 + t17469 + t17472 + t17475 - t17478 - t17482 - t17487 - t17490 - t17493 - t17496 + t17497 - t17499 - t17503 - t17505 - t17507;
    let t18309 = -t17509 - t17511 - t17513 - t17515 + t17517 + t17527 + t17530 - t17532 + t17534 - t17537 - t17542 + t17553 + t17555 + t17557 + t17559;
    (t18308, t18309)
}
