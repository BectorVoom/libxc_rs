//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 830/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk830<F: Float>(t164: F, t179: F, t2305: F, t4021: F, t4028: F, t744: F, t8096: F, t8101: F, t8394: F, t8404: F, t8407: F, t8413: F, t8416: F, t949: F, t953: F) -> F {
    let t8419 = F::cast_from(2.400108951976084_f64) * t4021 - F::cast_from(3.2915558116322368_f64) * t4028 + F::cast_from(1.2536914064583544_f64) * t8394 + F::cast_from(1.2536914064583544_f64) * t2305 * t744 + F::cast_from(1.2536914064583544_f64) * t2305 * t949 - F::cast_from(1.2536914064583544_f64) * t2305 * t953 - F::cast_from(0.04115066352984959_f64) * t164 * t8404 - F::cast_from(18.635258017632964_f64) * t8407 - F::cast_from(18.635258017632964_f64) * t179 * t8096 - F::cast_from(18.635258017632964_f64) * t179 * t8101 + F::cast_from(2.427516195194328_f64) * t8413 + F::cast_from(0.04115066352984959_f64) * t164 * t8416;
    t8419
}
