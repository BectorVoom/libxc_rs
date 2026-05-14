//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 816/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk816<F: Float>(t11458: F, t682: F, t1810: F, t1846: F, t1825: F, t5082: F, t5097: F, t696: F, t1806: F, t5105: F, t5100: F, t680: F, t1850: F, t5090: F, t5094: F, t5102: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11460 = 0.14055920378328537299e-1 * t11458 * t682;
    let t11461 = t1846 * t1810;
    let t11463 = t5082 * t1825;
    let t11465 = t696 * t5097;
    let t11467 = t1806 * t5105;
    let t11480 = 1.0 / t5100 / t680;
    let t11488 = t1850 * t5090;
    let t11491 = t696 * t5094;
    let t11493 = t1806 * t5102;
    (t11460, t11461, t11463, t11465, t11467, t11480, t11488, t11491, t11493)
}
