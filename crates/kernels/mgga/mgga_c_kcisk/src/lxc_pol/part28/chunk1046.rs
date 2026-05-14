//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1046/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1046<F: Float>(t24028: F, t5182: F, t2364: F, t6689: F, t10426: F, t23220: F, t6675: F, t5192: F, t6674: F, t1757: F, t5193: F, t7718: F, t6666: F, t4826: F, t8845: F, t1790: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t24029 = t5182 * t24028;
    let t24031 = t2364 * t6689;
    let t24032 = t10426 * t24031;
    let t24033 = t5182 * t24032;
    let t24035 = t6675 * t23220;
    let t24036 = t5192 * t24035;
    let t24037 = t6674 * t24036;
    let t24040 = t5193 * t7718 * t1757;
    let t24041 = t5192 * t24040;
    let t24042 = t5182 * t24041;
    let t24044 = t6666 * t23220;
    let t24045 = t5192 * t24044;
    let t24046 = t5182 * t24045;
    let t24048 = t8845 * t4826;
    let t24049 = t24048 * t1790;
    (t24029, t24031, t24033, t24035, t24037, t24040, t24042, t24044, t24046, t24048, t24049)
}
