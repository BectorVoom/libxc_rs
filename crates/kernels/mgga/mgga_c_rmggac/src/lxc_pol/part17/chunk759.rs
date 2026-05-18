//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 759/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk759<F: Float>(t1341: F, t357: F, t638: F, t7310: F, t7254: F, t7364: F, t7243: F, t1326: F, t2016: F, t7551: F, t2049: F, t35253: F, t7760: F) -> (F, F, F, F, F) {
    let t35633 = t638 * t7310 * t357 * t1341;
    let t35637 = t7254 * t7364;
    let t35654 = t7254 * t7243;
    let t35688 = t2016 * t7551 * t1326;
    let t35691 = t35688 * t2049 * t35253 * t7760;
    (t35633, t35637, t35654, t35688, t35691)
}
