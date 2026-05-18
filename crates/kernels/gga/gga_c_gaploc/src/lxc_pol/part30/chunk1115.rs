//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1115/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1115<F: Float>(t2558: F, t7589: F, t943: F, t2537: F, t7064: F, t7177: F, t1842: F, t21491: F, t883: F, t5538: F, t7305: F, t23296: F, t9647: F) -> (F, F, F, F, F, F) {
    let t29233 = F::new(0.64087718584518535698e-3) * t943 * t7589 * t2558;
    let t29242 = F::new(0.64087718584518535698e-3) * t7064 * t2537 * t7177;
    let t29273 = F::new(0.3845263115071112142e-2) * t7064 * t1842 * t883 * t21491;
    let t29277 = t5538 * t883;
    let t29280 = F::new(0.2563508743380741428e-2) * t7064 * t29277 * t7305;
    let t29304 = F::new(0.1281754371690370714e-2) * t9647 * t23296 * t2558;
    (t29233, t29242, t29273, t29277, t29280, t29304)
}
