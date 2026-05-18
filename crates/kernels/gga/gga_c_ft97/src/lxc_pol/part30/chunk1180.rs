//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1180/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1180<F: Float>(t10697: F, t142653: F, t143432: F, t1466: F, t154842: F, t193: F, t2404: F, t25412: F, t28868: F, t28934: F, t28940: F, t28946: F, t28992: F, t33966: F, t36011: F, t36060: F, t36063: F, t4309: F, t44601: F, t6216: F, t6386: F, t683: F, t684: F, t7114: F, t7585: F, t875: F) -> F {
    let t155066 = t1466 * t193 * t33966 * t28868 - t1466 * t193 * t7585 * t4309 / F::new(3.0) - F::new(12.0) * t10697 * t36060 * t875 + F::new(48.0) * t44601 * t36063 * t875 - F::new(24.0) * t10697 * t7114 * t6386 + F::new(2.0) / F::new(9.0) * t6216 * t25412 * t36011 * t684 - t6216 * t142653 * t28934 / F::new(3.0) + F::new(2.0) / F::new(9.0) * t6216 * t25412 * t28992 - F::new(2.0) / F::new(9.0) * t6216 * t683 * t7585 * t28940 + F::new(2.0) / F::new(27.0) * t6216 * t2404 * t7585 * t28946 + t143432 / F::new(9.0) - F::new(4.0) * t154842;
    t155066
}
