//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1219/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1219<F: Float>(t21125: F, t5968: F, t17594: F, t21130: F, t21134: F, t1392: F, t1979: F, t5441: F, t3751: F, t5427: F, t21106: F, t5976: F, t21110: F, t21073: F, t21078: F, t1419: F, t1961: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22582 = t5968 * t21125;
    let t22585 = t17594 * t21130;
    let t22588 = t5968 * t21134;
    let t22591 = t1392 * t1979;
    let t22592 = t22591 * t5441;
    let t22595 = t3751 * t1979;
    let t22596 = t22595 * t5427;
    let t22601 = t5976 * t21106;
    let t22604 = t5976 * t21110;
    let t22607 = t5976 * t21073;
    let t22610 = t5968 * t21078;
    let t22615 = t1961 * t1419;
    (t22582, t22585, t22588, t22592, t22596, t22601, t22604, t22607, t22610, t22615)
}
