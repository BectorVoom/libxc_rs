//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1067/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1067<F: Float>(t16404: F, t16471: F, t1882: F, t7073: F, t959: F, t11775: F, t28254: F, t11990: F, t2817: F, t11997: F, t2639: F, t188: F, t1903: F) -> (F, F, F, F, F) {
    let t33226 = t7073 * t1882 * t16471 * t959 * t16404;
    let t33228 = t11775 * t28254;
    let t33230 = t11990 * t2817;
    let t33232 = t11997 * t2639;
    let t33235 = t188 * t1903 * M_PI;
    (t33226, t33228, t33230, t33232, t33235)
}
