//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 746/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk746<F: Float>(t2192: F, t61: F, t650: F, t891: F, t3772: F, t7608: F, t3744: F, t3750: F, t7578: F, t7590: F, t8517: F, t8519: F, t8521: F, t8525: F, t8527: F, t8529: F, t8531: F, t8533: F, t8535: F) -> (F,) {
    let t8537 = t61 * t2192;
    let t8539 = t891 * t8537 * t650;
    let t8542 = t3772 * t7608;
    let t8548 = -3.600163427964126 * t8517 - 3.600163427964126 * t8519 - 22.07984838129906 * t8521 - 5.40024514194619 * t8525 - 3.600163427964126 * t8527 + 3.600163427964126 * t8529 - 3.600163427964126 * t8531 - 22.07984838129906 * t8533 + 1.6183441301295518 * t8535 - 1.1846959580306418 * t3744 * t8539 - 2.427516195194328 * t8542 - 2.427516195194328 * t3750 * t7590 - 4.855032390388656 * t3750 * t7578;
    (t8548,)
}
