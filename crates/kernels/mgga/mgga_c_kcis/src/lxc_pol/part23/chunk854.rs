//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 854/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk854<F: Float>(t16144: F, t5564: F, t659: F, t16050: F, t11409: F, t11411: F, t11413: F, t11415: F, t11455: F, t11457: F, t11460: F, t16048: F, t16062: F, t16088: F) -> (F, F, F) {
    let t16145 = F::new(0.21908444444444444444e0) * t16144;
    let t16146 = t659 * t5564;
    let t16156 = F::new(0.39862222222222222222e0) * t16050;
    let t16160 = -F::new(0.26574814814814814816e0) * t11409 + F::new(0.66437037037037037038e-1) * t11411 - F::new(0.19931111111111111111e0) * t11413 + F::new(0.99655555555555555557e-1) * t11415 + F::new(0.59793333333333333334e0) * t16088 + F::new(0.11958666666666666667e1) * t16062 + F::new(0.13287407407407407408e0) * t16048 - t16156 - F::new(0.18257037037037037037e0) * t11455 + F::new(0.54771111111111111111e-1) * t11457 + F::new(0.18257037037037037037e-1) * t11460;
    (t16145, t16146, t16160)
}
