//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 477/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk477<F: Float>(t1330: F, t2507: F, t306: F, t2143: F, t372: F, t1349: F, t1354: F, t1356: F, t1358: F, t1360: F, t2502: F, t2505: F, t2542: F, t2546: F) -> (F, F, F, F, F) {
    let t2636 = t2507 * t1330;
    let t2637 = t2636 * t306;
    let t2640 = t372 * t2143;
    let t2641 = t1349 * t2640;
    let t2648 = t1354 - F::cast_from(4.0_f64) * t2542 + t1356 + F::cast_from(4.0_f64) * t2546 + t1358 - F::cast_from(0.821419393556371_f64) * t2502 + t1360 + F::cast_from(0.821419393556371_f64) * t2505;
    (t2636, t2637, t2640, t2641, t2648)
}
