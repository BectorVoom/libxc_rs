//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 410/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk410<F: Float>(t1988: F, t606: F, t172: F, t5: F, t355: F, t435: F) -> (F, F, F) {
    let t1989 = t1988 * t606;
    let t1990 = 0.15724046144802076034e-3 * t1989;
    let t1991 = t5 * t172;
    let t1992 = t435 * t355;
    (t1990, t1991, t1992)
}
