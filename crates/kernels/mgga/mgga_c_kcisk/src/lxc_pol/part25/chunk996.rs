//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 996/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk996<F: Float>(t1737: F, t1746: F, t17624: F, t17399: F, t10937: F, t10941: F, t10944: F, t10947: F, t11105: F, t17379: F, t17382: F, t17402: F, t17405: F, t17408: F, t17412: F, t17417: F, t17420: F, t17435: F, t17458: F, t17463: F) -> (F, F) {
    let t17626 = t1737 * t17624 * t1746;
    let t17635 = 0.23744444444444444444e-1 * t17399;
    let t17645 = -t11105 - 0.15829629629629629629e-1 * t10937 + 0.39574074074074074073e-2 * t10941 - 0.11872222222222222222e-1 * t10944 + 0.5936111111111111111e-2 * t10947 - 0.79148148148148148146e-2 * t17382 + 0.79148148148148148146e-2 * t17402 - t17635 - 0.13059444444444444444e0 * t17379 - 0.19787037037037037037e-1 * t17408 + 0.71233333333333333332e-1 * t17458 + 0.47488888888888888888e-1 * t17412 - 0.11872222222222222222e-1 * t17405 - 0.10685e0 * t17463 - 0.14246666666666666666e0 * t17420 + 0.35616666666666666666e-1 * t17417 + 0.35616666666666666666e-1 * t17435;
    (t17626, t17645)
}
