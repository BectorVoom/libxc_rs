//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 668/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk668<F: Float>(t1191: F, t7785: F, t1172: F, t3679: F, t7753: F, t3677: F, t3683: F, t5668: F, t7738: F, t7742: F, t7746: F, t334: F, t2105: F) -> (F, F, F, F, F, F, F) {
    let t7786 = t7785 * t1191;
    let t7788 = 1.0 * t1172 * t7786;
    let t7789 = t7753 * t3679;
    let t7791 = 0.16081824322151104822e2 * t3677 * t7789;
    let t7796 = t3683 + 0.61805555555555555556e-2 * t5668 - 0.61805555555555555555e-2 * t7738 + 0.18541666666666666667e-1 * t7742 - 0.92708333333333333333e-2 * t7746;
    let t7797 = t7796 * t334;
    let t7802 = t2105 * t2105;
    (t7786, t7788, t7789, t7791, t7796, t7797, t7802)
}
