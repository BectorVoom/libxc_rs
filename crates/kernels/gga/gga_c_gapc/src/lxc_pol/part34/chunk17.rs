//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 17/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk17<F: Float>(t36: F, zeta_threshold: F) -> (F, F, F) {
    let t34 = F::new(1.0) <= zeta_threshold;
    let t37 = piecewise3::<f64>(t34, t36, F::new(1.0));
    let t40 = M_CBRT2;
    let t43 = F::new(1.0) / (F::new(2.0) * t40 - F::new(2.0));
    (t37, t40, t43)
}
