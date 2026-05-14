//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 566/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk566<F: Float>(t2337: F, t575: F, t574: F, t571: F, t1486: F, t2334: F, t1485: F, t2171: F, t799: F, t2329: F, t523: F, t522: F, t519: F, t1460: F, t2325: F, t1459: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2539 = t575 * t2337;
    let t2540 = t574 * t2539;
    let t2542 = 4.0 / 45.0 * t571 * t2540;
    let t2543 = t1486 * t2334;
    let t2544 = t1485 * t2543;
    let t2546 = 4.0 / 27.0 * t571 * t2544;
    let t2548 = 8.0 / 45.0 * t2171 * t799;
    let t2549 = t523 * t2329;
    let t2550 = t522 * t2549;
    let t2552 = 4.0 / 45.0 * t519 * t2550;
    let t2553 = t1460 * t2325;
    let t2554 = t1459 * t2553;
    (t2539, t2540, t2542, t2543, t2544, t2546, t2548, t2549, t2550, t2552, t2553, t2554)
}
