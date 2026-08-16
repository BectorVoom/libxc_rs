//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 541/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk541<F: Float>(t374: F, t376: F, t677: F, t370: F, t1032: F, t1036: F, t121: F, t1023: F, t248: F, t1020: F, t1017: F, t1030: F) -> (F, F, F, F, F, F, F) {
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / F::cast_from(13824.0_f64);
    let t3092 = t1032 * t1036;
    let t3101 = t121 * t376;
    let t3103 = t248 * t3101 * t1023;
    let t3104 = t1020 * t3103;
    let t3107 = t1030 * t1017;
    (t3082, t3084, t3092, t3101, t3103, t3104, t3107)
}
