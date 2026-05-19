//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 481/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk481<F: Float>(t2674: F, t93: F, t1372: F, t1374: F, t1376: F, t1378: F, t2502: F, t2505: F, t2542: F, t2546: F, t1381: F, t306: F) -> (F, F, F, F) {
    let t2675 = t93 * t2674;
    let t2688 = t1372 - F::new(6.25) * t2542 + t1374 + F::new(6.25) * t2546 + t1376 - F::cast_from(1.2466946262544771_f64) * t2502 + t1378 + F::cast_from(1.2466946262544771_f64) * t2505;
    let t2689 = t2688 * t1381;
    let t2690 = t2689 * t306;
    (t2675, t2688, t2689, t2690)
}
