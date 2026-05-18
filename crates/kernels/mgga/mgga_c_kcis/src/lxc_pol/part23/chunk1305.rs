//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1305/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1305<F: Float>(t98624: F, t27601: F, t28727: F, t98637: F, t16782: F, t18187: F, t27560: F, t27583: F, t27648: F, t28765: F, t95127: F, t98632: F, t98640: F, t98643: F, t98646: F) -> F {
    let t99504 = F::new(0.15476481481481481481e-2) * t98624;
    let t99506 = F::new(0.61782407407407407408e-3) * t28727 * t27601;
    let t99512 = F::new(0.15476481481481481481e-2) * t98637;
    let t99520 = F::new(0.11584201388888888889e-3) * t95127 + t99504 - t99506 - F::new(0.92673611111111111112e-3) * t28727 * t27648 + F::new(0.38691203703703703704e-2) * t98632 - F::new(0.92673611111111111112e-3) * t28727 * t27560 + t99512 - F::new(0.23214722222222222222e-2) * t98640 + F::new(0.92858888888888888886e-2) * t98643 - F::new(0.61905925925925925924e-2) * t98646 + F::new(0.61782407407407407408e-3) * t27583 * t18187 * t28765 * t16782;
    t99520
}
