//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1088/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1088<F: Float>(t1091: F, t142455: F, t142460: F, t142653: F, t142946: F, t2404: F, t25412: F, t2665: F, t28934: F, t28935: F, t28940: F, t28941: F, t28946: F, t28947: F, t28950: F, t28951: F, t28986: F, t33808: F, t6216: F, t683: F, t7612: F) -> F {
    let t152530 = -t33808 * t28947 / F::new(27.0) + t33808 * t28951 / F::new(9.0) + F::new(2.0) / F::new(9.0) * t6216 * t25412 * t28986 - t6216 * t2665 * t142460 * t1091 / F::new(9.0) - t6216 * t2665 * t142455 * t1091 / F::new(9.0) + t33808 * t28935 / F::new(9.0) + t33808 * t28941 / F::new(9.0) - t6216 * t2404 * t7612 * t28946 / F::new(27.0) + t6216 * t142946 * t28934 / F::new(9.0) + t6216 * t683 * t7612 * t28940 / F::new(9.0) + t6216 * t142946 * t28950 / F::new(9.0) - t6216 * t142653 * t28950 / F::new(3.0);
    t152530
}
