//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 790/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk790<F: Float>(t43: F, t50: F, t16225: F, t16231: F, t3331: F, t4565: F, t607: F, t6533: F, t16236: F, t16241: F, t3339: F, t4573: F, t611: F, t6547: F, zeta_threshold: F) -> (F,) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t16277 = piecewise3(t44, 0.0, 8.0 / 27.0 * t6533 * t16225 - 2.0 / 3.0 * t3331 * t4565 + 2.0 / 3.0 * t607 * t16231);
    let t16285 = piecewise3(t51, 0.0, 8.0 / 27.0 * t6547 * t16236 - 2.0 / 3.0 * t3339 * t4573 + 2.0 / 3.0 * t611 * t16241);
    let t16287 = t16277 / 2.0 + t16285 / 2.0;
    (t16287,)
}
