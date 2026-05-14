//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1043/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1043<F: Float>(t12439: F, t12441: F, t12446: F, t12448: F, t12450: F, t12452: F, t12454: F, t12457: F, t12459: F, t12461: F, t12463: F, t12466: F, t12469: F, t12473: F, t12476: F, t12479: F, t12484: F, t12488: F, t12491: F, t12493: F, t12496: F, t12500: F, t12504: F) -> (F, F) {
    let t14369 = t12439 + t12441 + t12446 - t12448 - t12450 + t12452 + t12454 - t12457 - t12459 - t12461 - t12463;
    let t14370 = -t12466 + t12469 + t12473 - t12476 + t12479 + t12484 + t12488 - t12491 + t12493 - t12496 + t12500 + t12504;
    (t14369, t14370)
}
