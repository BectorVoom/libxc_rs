//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1178/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1178<F: Float>(t142995: F, t1466: F, t1506: F, t154221: F, t193: F, t28978: F, t29035: F, t34022: F, t34025: F, t34262: F, t34312: F, t34326: F, t34330: F, t36273: F, t4129: F, t6222: F, t6963: F, t7024: F, t7581: F, t830: F) -> F {
    let t155009 = t6963 * t34326 / F::new(3.0) - t830 * t36273 + t6963 * t34022 - F::new(2.0) / F::new(3.0) * t6963 * t34025 - F::new(2.0) * t154221 + t6963 * t34262 / F::new(6.0) + t6963 * t34330 / F::new(3.0) + t34312 * t7024 / F::new(6.0) - t142995 / F::new(18.0) - F::new(2.0) / F::new(3.0) * t1466 * t193 * t6222 * t1506 * t4129 - t7581 * t28978 / F::new(3.0) - t7581 * t29035 / F::new(3.0);
    t155009
}
