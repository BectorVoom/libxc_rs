//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1091/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1091<F: Float>(t32879: F, t564: F, t1629: F, t9776: F, t9645: F, t9660: F, t9657: F, t12261: F, t2784: F) -> (F, F, F, F, F, F) {
    let t32880 = t564 * t32879;
    let t32881 = t32880 / 16.0;
    let t32882 = t1629 * t9776;
    let t32883 = t564 * t32882;
    let t32884 = t32883 / 8.0;
    let t32885 = t9645 * t9660;
    let t32887 = t9657 * t9660;
    let t32889 = t12261 * t2784;
    (t32881, t32882, t32884, t32885, t32887, t32889)
}
