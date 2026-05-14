//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1216/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1216<F: Float>(t4346: F, t6579: F, t1300: F, t397: F, t14608: F, t2306: F, t15093: F, t2339: F, t4534: F, t6602: F, t8432: F, t4169: F, t8185: F, t1610: F, t28034: F, t27402: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F) {
    let t55345 = t6579 * t4346;
    let t55867 = t1300 * t397;
    let t56817 = t2306 * t14608;
    let t57164 = t2339 * t15093;
    let t57167 = t6602 * t4534;
    let t75337 = t8432 * t4534;
    let t79107 = t8185 * t4169;
    let t79120 = t28034 * t1610;
    let t79161 = t27402 * sigma0;
    (t55345, t55867, t56817, t57164, t57167, t75337, t79107, t79120, t79161)
}
