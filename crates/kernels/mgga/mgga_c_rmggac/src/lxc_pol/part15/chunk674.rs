//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 674/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk674<F: Float>(t7562: F, t7767: F, t2181: F, t7561: F, t2165: F, t638: F, t7184: F, t2169: F, t1343: F, t7321: F, t1327: F, t4765: F, t640: F, t7352: F, t1295: F, t1302: F, t131: F, t20: F, t2018: F, t2020: F, t252: F) -> (F, F, F, F, F, F, F) {
    let t34613 = 0.10000709273223291967e0 * t7562;
    let t34649 = 0.91462949374725084942e-3 * t7767;
    let t34659 = t2181 * t7561;
    let t34662 = t638 * t7184 * t2165;
    let t34665 = t638 * t7184 * t2169;
    let t34683 = t7321 * t1343;
    let t34687 = t4765 * t34683 * t640 * t7352 * t1327;
    let t34688 = 0.10260057759007034251e-5 * t34687;
    let t34704 = t1295 * t1302 * t20 * t2018 * t2020 * t640 * t131 * t252;
    (t34613, t34649, t34659, t34662, t34665, t34688, t34704)
}
