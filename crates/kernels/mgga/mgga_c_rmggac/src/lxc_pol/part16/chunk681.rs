//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 681/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk681<F: Float>(t117: F, t1916: F, t30177: F, t321: F, t622: F, t2181: F, t7561: F, t2165: F, t638: F, t7184: F, t2169: F, t1343: F, t7321: F, t1327: F, t4765: F, t640: F, t7352: F) -> (F, F, F, F, F, F, F) {
    let t32556 = t1916 * t117;
    let t33228 = t30177 * t117;
    let t33235 = t622 * t321;
    let t34659 = t2181 * t7561;
    let t34662 = t638 * t7184 * t2165;
    let t34665 = t638 * t7184 * t2169;
    let t34683 = t7321 * t1343;
    let t34687 = t4765 * t34683 * t640 * t7352 * t1327;
    (t32556, t33228, t33235, t34659, t34662, t34665, t34687)
}
