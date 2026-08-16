//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 896/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk896<F: Float>(t1053: F, t3215: F, t390: F, t1376: F) -> (F, F, F, F) {
    let t10163 = t1053 * t1053;
    let t10164 = F::cast_from(1.0_f64) / t10163;
    let t11094 = F::cast_from(1.0_f64) / t3215 / t390;
    let t12019 = t1376 * t1376;
    let t12020 = F::cast_from(1.0_f64) / t12019;
    (t10164, t11094, t12019, t12020)
}
