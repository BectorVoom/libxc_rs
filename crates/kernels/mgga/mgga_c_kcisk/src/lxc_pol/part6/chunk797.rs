//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 797/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk797<F: Float>(t1800: F, t28788: F, t1799: F, t6697: F, t8514: F, t15858: F, t8480: F, t5182: F, t2441: F, t8786: F) -> (F, F, F, F) {
    let t28789 = t1800 * t28788;
    let t28790 = t1799 * t28789;
    let t28792 = t6697 * t8514;
    let t28793 = t1800 * t28792;
    let t28794 = t1799 * t28793;
    let t28796 = t15858 * t8480;
    let t28797 = t5182 * t28796;
    let t28800 = t2441 * t8786;
    (t28790, t28794, t28797, t28800)
}
