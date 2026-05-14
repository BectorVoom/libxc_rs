//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1196/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1196<F: Float>(t1517: F, t1518: F, t16073: F, t5976: F, t1392: F, t1455: F, t5441: F, t16082: F, t16078: F, t14955: F, t5977: F, t5969: F, t3751: F, t5427: F, t16069: F, t5968: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17565 = t1517 * t1518;
    let t17568 = t5976 * t16073;
    let t17571 = t1392 * t1455;
    let t17572 = t17571 * t5441;
    let t17575 = t5976 * t16082;
    let t17578 = t5976 * t16078;
    let t17583 = t14955 * t5977;
    let t17586 = 0.5895802469135802469e-1 * t14955 * t5969;
    let t17587 = t3751 * t1455;
    let t17588 = t17587 * t5427;
    let t17591 = t5968 * t16069;
    (t17565, t17568, t17572, t17575, t17578, t17583, t17586, t17588, t17591)
}
