//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1406/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1406<F: Float>(t1892: F, t5744: F, t786: F, t1320: F, t13632: F, t1317: F, t3857: F, t5569: F, t1856: F, t512: F, t9544: F, t5571: F, t9387: F) -> (F, F, F, F, F, F) {
    let t48083 = t5744 * t1892;
    let t48084 = t786 * t48083;
    let t48152 = t1320 * t13632;
    let t48225 = t1317 * t13632;
    let t48227 = t3857 * t5569;
    let t48243 = t512 * t1856 * t9544;
    let t48262 = t5571 * t9387;
    (t48084, t48152, t48225, t48227, t48243, t48262)
}
