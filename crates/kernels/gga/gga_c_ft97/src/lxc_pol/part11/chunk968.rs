//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 968/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk968<F: Float>(t549: F, t554: F, t8153: F, t8157: F, t1355: F, t1995: F, t2001: F, t2002: F, t2059: F, t2071: F, t3392: F, t39824: F, t39828: F, t399: F, t40093: F, t539: F, t555: F, t5802: F, t5818: F, t8807: F, t8812: F, t8865: F, t8877: F, t8885: F, t8894: F, t8907: F, t8932: F) -> F {
    let t40150 = t549 * t8153 * t8157 * t554;
    let t40164 = F::new(0.22445349300913785316e3) * t5802 * t39824 - F::new(0.11222674650456892658e3) * t1355 * t39828 - F::new(36.0) * t3392 * t8907 * t2059 * t2071 + F::new(8.0) * t3392 * t8877 * t8932 - F::new(8.0) * t2001 * t2002 * t8932 - F::new(12.0) * t2001 * t8865 * t2071 + F::new(0.13035593825592482769e1) * t5818 * t40150 - F::new(0.43451979418641609231e0) * t3392 * t40150 - F::new(0.11093760908123778558e3) * t8812 * t8807 * t539 + F::new(0.14498192132169191472e2) * t1995 * t40093 * t8885 + F::new(0.14498192132169191472e2) * t8894 * t555 * t399;
    t40164
}
