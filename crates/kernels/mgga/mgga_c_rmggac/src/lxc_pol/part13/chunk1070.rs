//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1070/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1070<F: Float>(t40259: F, t1356: F, t2211: F, t27044: F, t29892: F, t31125: F, t35655: F, t35665: F, t37423: F, t40214: F, t40217: F, t40222: F, t40227: F, t40232: F, t40237: F, t40242: F, t40247: F, t40250: F, t40254: F, t5888: F, t739: F, t884: F) -> F {
    let t43338 = F::new(0.36366215538993788974e-1) * t40259;
    let t43346 = -F::new(0.11974241701863808564e0) * t884 * t2211 * t31125 - F::new(0.23948483403727617128e0) * t1356 * t37423 * t5888 - F::new(0.1440846329149835838e-2) * t40214 - F::new(0.1440846329149835838e-2) * t40217 - F::new(0.638468998399467591e-4) * t40222 - F::new(0.212822999466489197e-4) * t40227 + F::new(0.212822999466489197e-4) * t40232 - F::new(0.14365552463988020797e-3) * t40237 - F::new(0.47885174879960069324e-4) * t40242 + F::new(0.47885174879960069324e-4) * t40247 - F::new(0.49658699875514145966e-4) * t40250 + F::new(0.5107751987195740728e-4) * t40254 + F::new(0.39726959900411316772e-4) * t35655 + t43338 + F::new(0.11918087970123395032e-3) * t35665 + F::new(0.23948483403727617128e0) * t739 * t2211 * t29892 - F::new(0.23948483403727617128e0) * t884 * t2211 * t27044;
    t43346
}
