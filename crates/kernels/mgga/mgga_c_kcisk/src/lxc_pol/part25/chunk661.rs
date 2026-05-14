//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 661/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk661<F: Float>(t2372: F, t708: F, t1648: F, t7028: F, t682: F, t1824: F, t4629: F, t1882: F, t6771: F, t706: F, t1417: F, t2522: F, t2487: F, t695: F, t1060: F, t4609: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t7029 = t708 * t2372;
    let t7030 = t7029 * t1648;
    let t7031 = t7028 * t7030;
    let t7034 = t682 * t2372;
    let t7035 = t7034 * t1824;
    let t7036 = t4629 * t7035;
    let t7039 = t1882 * t6771;
    let t7040 = t706 * t7039;
    let t7043 = t1417 * t2522;
    let t7045 = t2487 * t695;
    let t7046 = t7045 * t1060;
    let t7047 = t4609 * t7046;
    (t7029, t7030, t7031, t7034, t7035, t7036, t7039, t7040, t7043, t7045, t7047)
}
