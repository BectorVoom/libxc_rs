//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 796/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk796<F: Float>(t151: F, t7991: F, t143: F, t3277: F, t3287: F, t3292: F, t3300: F, t4061: F, t4072: F, t4581: F, t7578: F, t7590: F, t7598: F, t7602: F, t7962: F, t7974: F, t7981: F, t7989: F) -> F {
    let t7992 = t151 * t7991;
    let t7997 = F::cast_from(3.7610742193750633_f64) * t143 * t7962 + F::cast_from(2.2140749178833072_f64) * t7974 * t4061 + F::cast_from(18.635258017632964_f64) * t4581 * t7590 + F::cast_from(37.27051603526593_f64) * t4581 * t7578 - F::cast_from(2.2140749178833072_f64) * t7981 - F::cast_from(4.4281498357666145_f64) * t4072 * t7598 - F::cast_from(2.2140749178833072_f64) * t4072 * t7602 + F::cast_from(0.04115066352984959_f64) * t7989 + F::cast_from(1.2536914064583544_f64) * t7992 + F::cast_from(1.6183441301295518_f64) * t3277 + F::cast_from(2.427516195194328_f64) * t3287 + t3292 - F::cast_from(1.8805371096875316_f64) * t3300;
    t7997
}
