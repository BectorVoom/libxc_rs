//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 810/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk810<F: Float>(t161: F, t8141: F, t200: F, t3667: F, t3668: F, t3670: F, t3744: F, t7706: F, t7768: F, t7776: F, t7962: F, t8117: F, t8121: F, t8124: F, t8129: F, t8131: F) -> F {
    let t8142 = t161 * t8141;
    let t8144 = t3667 + F::cast_from(1.2536914064583544_f64) * t3668 + F::cast_from(1.2536914064583544_f64) * t3670 - F::cast_from(19.489173774580152_f64) * t8117 + F::cast_from(22.07984838129906_f64) * t8121 + F::cast_from(1.1846959580306418_f64) * t3744 * t8124 - F::cast_from(4.738783832122567_f64) * t8129 + F::cast_from(3.2915558116322368_f64) * t8131 + F::cast_from(2.427516195194328_f64) * t200 * t7962 + F::cast_from(2.427516195194328_f64) * t200 * t7768 + F::cast_from(2.427516195194328_f64) * t200 * t7776 + F::cast_from(2.427516195194328_f64) * t200 * t7706 + F::cast_from(3.2915558116322368_f64) * t8142;
    t8144
}
