//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 932/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk932<F: Float>(t11417: F, t116: F, t128: F, t1672: F, t1906: F, t515: F, t644: F, t19: F, t169: F, t3665: F, t8652: F) -> (F, F, F, F, F, F, F, F) {
    let t11418 = t116 * t11417;
    let t11420 = t1906 * t1672 * t128;
    let t11421 = t11418 * t11420;
    let t11423 = t515 * t644;
    let t11424 = t11423 * t19;
    let t11425 = t169 * t11424;
    let t11426 = t11425 * t3665;
    let t11428 = F::cast_from(1.0_f64) / t8652;
    (t11418, t11420, t11421, t11423, t11424, t11425, t11426, t11428)
}
