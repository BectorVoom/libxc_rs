//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 992/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk992<F: Float>(t2404: F, t4857: F, t2418: F, t4859: F, t1724: F, t7135: F, t4903: F, t7138: F, t4911: F, t7134: F, t10928: F, t2417: F, t4928: F, t7175: F, t7157: F, t10902: F, t2430: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17520 = t2404 * t4857;
    let t17523 = t2418 * t4859;
    let t17530 = t7135 * t1724;
    let t17533 = t2418 * t4903;
    let t17536 = t7138 * t4859;
    let t17539 = t7134 * t4911;
    let t17540 = t17539 * t1724;
    let t17543 = t7138 * t4903;
    let t17546 = t2417 * t10928;
    let t17547 = t17546 * t4859;
    let t17552 = t4928 * t7175;
    let t17553 = t17552 * t7157;
    let t17556 = t10902 * t2430;
    (t17520, t17523, t17530, t17533, t17536, t17540, t17543, t17547, t17553, t17556)
}
