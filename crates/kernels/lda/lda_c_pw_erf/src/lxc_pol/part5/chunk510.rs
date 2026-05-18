//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 510/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk510<F: Float>(t2146: F, t826: F, t2337: F, t575: F, t574: F, t571: F, t1486: F, t2334: F, t1485: F, t2171: F, t799: F, t2329: F, t523: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2538 = F::new(8.0) / F::new(45.0) * t2146 * t826;
    let t2539 = t575 * t2337;
    let t2540 = t574 * t2539;
    let t2542 = F::new(4.0) / F::new(45.0) * t571 * t2540;
    let t2543 = t1486 * t2334;
    let t2544 = t1485 * t2543;
    let t2546 = F::new(4.0) / F::new(27.0) * t571 * t2544;
    let t2548 = F::new(8.0) / F::new(45.0) * t2171 * t799;
    let t2549 = t523 * t2329;
    (t2538, t2539, t2540, t2542, t2543, t2544, t2546, t2548, t2549)
}
