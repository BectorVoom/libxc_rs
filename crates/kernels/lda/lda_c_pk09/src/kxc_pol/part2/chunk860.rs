//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 860/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk860<F: Float>(t1240: F, t2640: F, t1349: F, t1337: F, t623: F, t5279: F, t1364: F, t2143: F, t2649: F, t310: F, t2688: F, t6175: F, t306: F, t1380: F, t309: F, t1336: F, t2689: F) -> (F, F, F, F, F, F, F) {
    let t10407 = t2640 * t1240;
    let t10408 = t1349 * t10407;
    let t10409 = t1337 * t10408;
    let t10411 = t2640 * t623;
    let t10412 = t5279 * t10411;
    let t10415 = t1364 * t2143;
    let t10416 = t1349 * t10415;
    let t10419 = t2649 * t1240;
    let t10420 = t310 * t10419;
    let t10421 = t1337 * t10420;
    let t10423 = t2688 * t6175;
    let t10424 = t10423 * t306;
    let t10426 = t309 * t310 * t1380;
    let t10429 = t2689 * t1336;
    (t10409, t10412, t10416, t10421, t10424, t10426, t10429)
}
