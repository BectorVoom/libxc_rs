//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1089/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1089<F: Float>(t12345: F, t4190: F, t8207: F, t28570: F, t39301: F, t17311: F, t27509: F, t12338: F, t28580: F, t17708: F, t2253: F, t4189: F, t17323: F, t27494: F, t27503: F, t48058: F) -> (F, F, F, F, F, F, F) {
    let t97852 = 6.0 * t12345 * t8207 * t4190;
    let t97854 = 12.0 * t39301 * t28570;
    let t97856 = 2.0 * t17311 * t27509;
    let t97862 = 4.0 * t12338 * t28580;
    let t97870 = 2.0 * t4189 * t2253 * t17708;
    let t97875 = 4.0 * t27494 * t17323;
    let t97877 = 6.0 * t48058 * t27503;
    (t97852, t97854, t97856, t97862, t97870, t97875, t97877)
}
