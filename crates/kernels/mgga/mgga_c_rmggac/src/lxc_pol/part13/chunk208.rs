//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 208/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk208<F: Float>(t31: F, t830: F, t309: F, t313: F, t804: F, t811: F, t817: F, t822: F, t826: F, t87: F, t91: F, t98: F) -> (F, F, F) {
    let t831 = t31 * t830;
    let t832 = F::new(22.0) / F::new(9.0) * t831;
    let t833 = F::new(80.0) / F::new(9.0) * t804 * t91 - F::new(100.0) / F::new(9.0) * t309 * t313 + F::new(20.0) / F::new(9.0) * t87 * t811 + F::new(10.0) / F::new(3.0) * t87 * t817 + F::new(20.0) / F::new(9.0) * t98 * t822 + F::new(10.0) / F::new(3.0) * t98 * t826 - t832;
    (t831, t832, t833)
}
