//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 827/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk827<F: Float>(t4998: F, t5498: F, t2013: F, t10879: F, t2015: F, t5502: F, t10886: F, t5487: F, t5471: F, t5480: F, t5464: F, t1772: F, t10487: F, t786: F, t2005: F, t5483: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t12174 = t4998 * t5498;
    let t12175 = t2013 * t12174;
    let t12179 = t10879 * t2015;
    let t12180 = t2013 * t12179;
    let t12182 = t4998 * t5502;
    let t12183 = t2013 * t12182;
    let t12185 = t10886 * t5487;
    let t12186 = t2013 * t12185;
    let t12188 = t5471 * t5480;
    let t12194 = t5464 * sigma2;
    let t12195 = t12194 * t1772;
    let t12198 = t786 * t10487;
    let t12230 = t2005 * t5483;
    (t12175, t12180, t12183, t12186, t12188, t12195, t12198, t12230)
}
