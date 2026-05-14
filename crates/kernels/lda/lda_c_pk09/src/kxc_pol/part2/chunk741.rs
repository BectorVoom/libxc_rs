//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 741/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk741<F: Float>(t2318: F, t633: F, t849: F, t719: F, t8092: F, t179: F, t8141: F, t192: F, t7991: F, t1067: F, t2239: F, t164: F, t200: F, t2341: F, t704: F, t790: F, t8096: F, t8101: F) -> (F,) {
    let t8421 = t849 * t2318 * t633;
    let t8428 = t719 * t8092;
    let t8436 = t179 * t8141;
    let t8440 = t192 * t7991;
    let t8442 = t2239 * t1067;
    let t8444 = t179 * t7991;
    let t8446 = t192 * t8141;
    let t8448 = 0.04115066352984959 * t164 * t8421 - 2.2140749178833072 * t704 * t2341 - 2.427516195194328 * t790 * t2341 + 2.2140749178833072 * t8428 + 2.2140749178833072 * t192 * t8096 + 2.2140749178833072 * t192 * t8101 + 2.427516195194328 * t200 * t8101 + 12.423505345088643 * t8436 + 2.427516195194328 * t200 * t8096 - 1.4760499452555382 * t8440 + 1.4760499452555382 * t8442 + 12.423505345088643 * t8444 - 1.4760499452555382 * t8446;
    (t8448,)
}
