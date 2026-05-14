//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 647/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk647<F: Float>(t1664: F, t6834: F, t1645: F, t2381: F, t4744: F, t1663: F, t4742: F, t4638: F, t4748: F, t6756: F, t6761: F, t6766: F, t6769: F, t600: F, t2386: F, t45: F) -> (F, F, F, F, F, F, F, F) {
    let t6835 = t6834 * t1664;
    let t6837 = 1.0 * t1645 * t6835;
    let t6838 = t2381 * t4744;
    let t6839 = t6838 * t1663;
    let t6841 = 0.16081824322151104822e2 * t4742 * t6839;
    let t6847 = t4748 + 0.30902777777777777778e-2 * t4638 + 0.30902777777777777778e-2 * t6756 - 0.61805555555555555555e-2 * t6761 + 0.18541666666666666667e-1 * t6766 + 0.18541666666666666667e-1 * t6769;
    let t6848 = t6847 * t600;
    let t6851 = t45 * t2386;
    (t6835, t6837, t6838, t6839, t6841, t6847, t6848, t6851)
}
