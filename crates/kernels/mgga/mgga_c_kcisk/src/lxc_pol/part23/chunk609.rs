//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 609/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk609<F: Float>(t1483: F, t1497: F, t4224: F, t4227: F, t4233: F, t4238: F, t4242: F, t4298: F, t4302: F, t4307: F, t4310: F, t4314: F, t4316: F, t4222: F) -> (F, F) {
    let t4318 = t1483 * t1497;
    let t4320 = t4224 / 128.0 - t4227 / 24.0 + t4233 / 96.0 - t4238 / 128.0 - t4242 / 72.0 + t4298 / 16.0 - t4302 / 256.0 - t4307 / 576.0 - 2.0 / 9.0 * t4310 + 11.0 / 18.0 * t4314 - t4316 / 3.0 + t4318 / 12.0;
    let t4321 = t4222 + t4320;
    (t4318, t4321)
}
