//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 984/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk984<F: Float>(t1091: F, t140490: F, t140495: F, t140508: F, t140513: F, t149674: F, t2354: F, t2404: F, t24204: F, t28012: F, t28015: F, t28027: F, t28032: F, t28038: F, t28042: F, t3051: F, t33496: F, t33499: F, t33537: F, t35259: F, t6002: F, t683: F, t7436: F, t7441: F) -> F {
    let t149700 = t33499 * t28027 / F::new(9.0) - F::new(2.0) / F::new(9.0) * t6002 * t683 * t7441 * t28032 + F::new(2.0) / F::new(27.0) * t6002 * t2404 * t7441 * t28038 + t149674 / F::new(54.0) - t28015 * t33537 / F::new(18.0) - t6002 * t140513 * t28042 / F::new(3.0) + t28015 * t33496 / F::new(9.0) - t24204 * t35259 / F::new(9.0) - t6002 * t2354 * t140490 * t1091 / F::new(9.0) - t6002 * t2354 * t140508 * t1091 / F::new(9.0) + t7436 * t3051 * t28012 / F::new(9.0) - t6002 * t2354 * t140495 * t1091 / F::new(18.0);
    t149700
}
