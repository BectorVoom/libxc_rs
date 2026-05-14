//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 964/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk964<F: Float>(t17095: F, t1869: F, t4648: F, t6697: F, t1800: F, t1799: F, t4640: F, t5054: F, t5068: F, t6965: F, t1873: F, t2441: F, t4797: F, t1899: F, t4581: F, t6708: F) -> (F, F, F, F, F, F, F) {
    let t17096 = t1869 * t17095;
    let t17098 = t6697 * t4648;
    let t17099 = t1800 * t17098;
    let t17100 = t1799 * t17099;
    let t17102 = t6697 * t4640;
    let t17103 = t1800 * t17102;
    let t17104 = t5054 * t17103;
    let t17107 = t6965 * t5068;
    let t17108 = t1873 * t17107;
    let t17109 = t1869 * t17108;
    let t17111 = t2441 * t4797;
    let t17112 = t1899 * t17111;
    let t17113 = t1873 * t17112;
    let t17114 = t1869 * t17113;
    let t17116 = t4581 * t6708;
    (t17096, t17100, t17104, t17109, t17111, t17114, t17116)
}
