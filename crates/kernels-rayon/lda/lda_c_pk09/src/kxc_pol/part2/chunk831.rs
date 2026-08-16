//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 831/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk831(t2318: f64, t633: f64, t849: f64, t719: f64, t8092: f64, t179: f64, t8141: f64, t192: f64, t7991: f64, t1067: f64, t2239: f64, t164: f64, t200: f64, t2341: f64, t704: f64, t790: f64, t8096: f64, t8101: f64) -> f64 {
    let t8421 = t849 * t2318 * t633;
    let t8428 = t719 * t8092;
    let t8436 = t179 * t8141;
    let t8440 = t192 * t7991;
    let t8442 = t2239 * t1067;
    let t8444 = t179 * t7991;
    let t8446 = t192 * t8141;
    let t8448 = 0.04115066352984959_f64 * t164 * t8421 - 2.2140749178833072_f64 * t704 * t2341 - 2.427516195194328_f64 * t790 * t2341 + 2.2140749178833072_f64 * t8428 + 2.2140749178833072_f64 * t192 * t8096 + 2.2140749178833072_f64 * t192 * t8101 + 2.427516195194328_f64 * t200 * t8101 + 12.423505345088643_f64 * t8436 + 2.427516195194328_f64 * t200 * t8096 - 1.4760499452555382_f64 * t8440 + 1.4760499452555382_f64 * t8442 + 12.423505345088643_f64 * t8444 - 1.4760499452555382_f64 * t8446;
    t8448
}
