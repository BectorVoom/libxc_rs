//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 796/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk796<F: Float>(t7344: F, t7932: F, t7936: F, t14267: F, t71: F, t132: F, t270: F, t31: F, t35688: F, t1323: F, t1326: F, t35253: F, t68: F) -> (F, F, F, F, F, F) {
    let t36935 = t7344 * t7932;
    let t36936 = t36935 * t7936;
    let t36938 = t14267 * t71;
    let t36940 = t132 * t270 * t31;
    let t36942 = t35688 * t36938 * t36940;
    let t36943 = F::cast_from(0.13010691197123848594e-3_f64) * t36942;
    let t36945 = t1323 * t1326 * t14267;
    let t36948 = t36945 * t35253 * t68 * t36940;
    (t36935, t36936, t36940, t36943, t36945, t36948)
}
