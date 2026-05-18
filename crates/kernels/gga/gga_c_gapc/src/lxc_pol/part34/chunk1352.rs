//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1352/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1352<F: Float>(t35404: F, t35406: F, t35409: F, t35412: F, t35397: F, t36332: F, t36333: F, t36334: F, t36335: F, t36336: F, t36337: F, t36338: F, t36340: F) -> F {
    let t36341 = F::new(0.17379648562707520765e-3) * t35404;
    let t36342 = F::new(0.14024275817241799902e-4) * t35406;
    let t36343 = F::new(0.2530696388073708253e-5) * t35409;
    let t36344 = F::new(0.14762395597096631476e-5) * t35412;
    let t36345 = t36332 - t36333 - t36334 - t36335 - t36336 - t36337 - t36338 + F::new(0.53949325746737929042e-3) * t35397 - t36340 - t36341 + t36342 - t36343 - t36344;
    t36345
}
