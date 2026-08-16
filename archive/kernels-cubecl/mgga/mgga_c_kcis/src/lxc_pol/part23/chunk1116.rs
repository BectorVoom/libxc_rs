//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1116/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1116<F: Float>(t28683: F, t28697: F, t1307: F, t6207: F, t6159: F, t1615: F, t2109: F, t27596: F, t6176: F, t251: F, t6193: F, t1598: F) -> (F, F, F, F, F, F, F, F) {
    let t28698 = t28683 + t28697;
    let t28700 = t6207 * t1307;
    let t28701 = t6159 * t28700;
    let t28706 = t2109 * t1615;
    let t28707 = t27596 * t28706;
    let t28708 = t6176 * t28707;
    let t28713 = t6193 * t251;
    let t28714 = t28713 * t1598;
    (t28698, t28700, t28701, t28706, t28707, t28708, t28713, t28714)
}
