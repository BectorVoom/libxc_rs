//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1005/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1005<F: Float>(t1446: F, t5360: F, t5397: F, t1318: F, t1466: F, t2156: F, t3563: F, t3770: F, t4763: F, t2146: F, t3763: F, t3900: F, t3799: F, t4738: F, t13453: F, t13458: F, t13463: F, t13465: F, t13466: F, t13467: F) -> (F, F, F, F, F, F, F, F) {
    let t13469 = 4.0 / 5.0 * t1446 * t5360;
    let t13470 = t1446 * t5397;
    let t13471 = 16.0 / 15.0 * t13470;
    let t13475 = 4.0 / 15.0 * t1318 * t1466 * t2156 * t3563;
    let t13477 = 4.0 / 5.0 * t4763 * t3770;
    let t13478 = t2146 * t3763;
    let t13479 = 8.0 / 135.0 * t13478;
    let t13480 = t4763 * t3900;
    let t13481 = 16.0 / 15.0 * t13480;
    let t13483 = 4.0 / 5.0 * t4738 * t3799;
    let t13484 = -t13453 + t13458 + t13463 - t13465 + t13466 - t13467 + t13469 + t13471 - t13475 - t13477 - t13479 - t13481 - t13483;
    (t13469, t13471, t13475, t13477, t13479, t13481, t13483, t13484)
}
