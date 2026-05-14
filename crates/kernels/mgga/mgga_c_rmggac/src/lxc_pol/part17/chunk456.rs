//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 456/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk456<F: Float>(t5432: F, t5434: F, t5444: F, t5446: F, t5448: F, t4366: F, t5465: F, t5467: F, t4372: F, t4290: F, t4324: F, t4328: F, t4361: F, t4365: F, t5464: F, t5471: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6006 = 0.34631718211362927517e2 * t5432;
    let t6007 = 0.48830526149350786811e-3 * t5434;
    let t6008 = 0.21687162600603479684e-1 * t5444;
    let t6009 = 40.0 * t5446;
    let t6010 = 24.0 * t5448;
    let t6011 = 8.0 * t4366;
    let t6012 = 0.23392894490538584828e1 * t5465;
    let t6013 = 0.11696447245269292414e1 * t5467;
    let t6014 = 8.0 * t4372;
    let t6015 = t4290 - t6006 + t6007 + t4361 - t4365 + t6008 + t6009 - t6010 + t4324 - t6011 + t4328 - t5464 + t6012 - t6013 + t5471 - t6014;
    (t6006, t6007, t6008, t6009, t6010, t6011, t6012, t6013, t6014, t6015)
}
