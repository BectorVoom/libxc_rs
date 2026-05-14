//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 947/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk947<F: Float>(t107: F, t78: F, t2032: F, t5679: F, t6081: F, t1980: F, t6110: F, t1387: F, t60: F) -> (F, F, F, F, F) {
    let t14630 = t107 * t78;
    let t14667 = t5679 * t2032;
    let t15349 = t6081 * t2032;
    let t15362 = t1980 * t6110;
    let t15478 = t60 * t1387;
    (t14630, t14667, t15349, t15362, t15478)
}
