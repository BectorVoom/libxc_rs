//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 920/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk920<F: Float>(t686: F, t695: F, t11525: F, t11529: F, t11564: F, t11566: F, t11574: F, t11578: F, t11586: F, t11588: F, t11605: F, t11607: F, t11613: F, t11615: F, t11623: F, t11626: F, t11633: F, t11635: F, t429: F, t435: F, t445: F, t4593: F, t5126: F, t5134: F, t6910: F) -> (F,) {
    let t16286 = t686 * t695;
    let t16293 = -0.117630625e-4 * t11564 + 0.15684083333333333333e-4 * t11566 + 0.4684e-2 * t11574 - 0.15613333333333333333e-2 * t11578 - 0.13208333333333333333e-2 * t11586 + 0.88055555555555555553e-3 * t11588 + 0.10359077815592613752e-3 * t6910 + 0.47822877300252710492e-1 * t11605 - 0.11955719325063177623e-1 * t11607 - 0.62154466893555682512e-3 * t11613 + 0.10359077815592613752e-3 * t11615 + 0.7026e-2 * t429 * t5126 - 0.1585e-2 * t435 * t11525 * t4593 - 0.10082625e-4 * t445 * t11529 * t16286 + 0.23911438650126355246e-1 * t11623 - 0.31077233446777841256e-3 * t11626 + t11633 - t11635 - 0.23911438650126355246e-1 * t5134;
    (t16293,)
}
