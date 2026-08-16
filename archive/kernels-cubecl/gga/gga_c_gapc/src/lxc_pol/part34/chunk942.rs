//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 942/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk942<F: Float>(t9595: F, t9597: F, t9600: F, t9603: F, t9607: F, t9610: F, t9614: F, t9616: F, t9618: F, t9621: F, t9625: F, t9628: F, t9631: F) -> F {
    let t10886 = F::cast_from(0.57970906942607043472e-5_f64) * t9595 - F::cast_from(0.57970906942607043472e-5_f64) * t9597 + F::cast_from(0.86956360413910565208e-5_f64) * t9600 - F::cast_from(0.12380169846338434109e-5_f64) * t9603 + F::cast_from(0.10136107947527008247e-3_f64) * t9607 - F::cast_from(0.34752370105806885418e-3_f64) * t9610 - F::cast_from(0.34752370105806885418e-3_f64) * t9614 - F::cast_from(0.24326659074064819793e-2_f64) * t9616 + F::cast_from(0.84540905957968605064e-6_f64) * t9618 - F::cast_from(0.27801896084645508334e-2_f64) * t9621 + F::cast_from(0.20240885416666666668e-4_f64) * t9625 + F::cast_from(0.10120442708333333334e-3_f64) * t9628 + F::cast_from(0.10120442708333333334e-3_f64) * t9631;
    t10886
}
