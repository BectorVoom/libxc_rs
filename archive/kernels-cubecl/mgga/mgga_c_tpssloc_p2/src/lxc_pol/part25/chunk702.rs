//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 702/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk702<F: Float>(t533: F, t7216: F, t1390: F, t2095: F, t6999: F, t113: F, t1266: F, t1393: F, t1983: F, t2036: F, t2040: F, t2075: F, t2079: F, t2096: F, t2314: F, t4034: F, t510: F, t574: F, t650: F, t652: F, t672: F, t6876: F, t7040: F, t7042: F, t7050: F, t7057: F, t7061: F, t7156: F, t7166: F, t7171: F) -> (F, F, F, F) {
    let t7217 = t533 * t7216;
    let t7218 = t7217 * t1390;
    let t7220 = t2095 * t6999;
    let t7222 = -t113 * t7156 - t1266 * t2036 + t1393 * t2079 + F::cast_from(3.0_f64) * t1983 * t7171 + t1983 * t7218 - t1983 * t7220 - F::cast_from(2.0_f64) * t2040 * t2314 - F::cast_from(2.0_f64) * t2040 * t4034 - t2075 * t650 + t2096 * t6876 - t510 * t7040 + t574 * t7166 - F::cast_from(2.0_f64) * t652 * t7050 - F::cast_from(2.0_f64) * t652 * t7057 - F::cast_from(2.0_f64) * t652 * t7061 - F::cast_from(2.0_f64) * t672 * t7042;
    (t7217, t7218, t7220, t7222)
}
