//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 19/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk19<F: Float>(t11: F, t14: F, t17: F, t25: F) -> (F, F, F) {
    let t51 = F::new(0.51785e1) * t14 + F::new(0.905775e0) * t11 + F::new(0.1100325e0) * t17 + F::new(0.1241775e0) * t25;
    let t54 = F::new(1.0) + F::cast_from(0.29608574643216675549e2_f64) / t51;
    let t55 = F::ln(t54);
    (t51, t54, t55)
}
