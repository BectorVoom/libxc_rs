//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 895/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk895<F: Float>(t9595: F, t9597: F, t9600: F, t9603: F, t9607: F, t9610: F, t9614: F, t9616: F, t9618: F, t9621: F, t9625: F, t9628: F, t9631: F) -> F {
    let t10886 = F::new(0.57970906942607043472e-5) * t9595 - F::new(0.57970906942607043472e-5) * t9597 + F::new(0.86956360413910565208e-5) * t9600 - F::new(0.12380169846338434109e-5) * t9603 + F::new(0.10136107947527008247e-3) * t9607 - F::new(0.34752370105806885418e-3) * t9610 - F::new(0.34752370105806885418e-3) * t9614 - F::new(0.24326659074064819793e-2) * t9616 + F::new(0.84540905957968605064e-6) * t9618 - F::new(0.27801896084645508334e-2) * t9621 + F::new(0.20240885416666666668e-4) * t9625 + F::new(0.10120442708333333334e-3) * t9628 + F::new(0.10120442708333333334e-3) * t9631;
    t10886
}
