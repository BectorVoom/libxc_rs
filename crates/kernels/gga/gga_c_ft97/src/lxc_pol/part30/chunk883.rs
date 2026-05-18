//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 883/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk883<F: Float>(t1466: F, t1526: F, t2: F, t2320: F, t342: F, t34289: F, t343: F, t34301: F, t36071: F, t36075: F, t36080: F, t36086: F, t7079: F, t7084: F, t7570: F, t7571: F) -> F {
    let t36091 = (-t36071 * t7571 / F::new(6.0) + t34289 + t1466 * t36075 / F::new(18.0) + t1466 * t7084 / F::new(3.0) - t7570 * t36080 / F::new(6.0) - t34301 - t1526 * t2320 * t7079 / F::new(12.0) - t342 * t343 * t36086 / F::new(4.0)) * t2;
    t36091
}
