//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 969/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk969<F: Float>(t1248: F, t8201: F, t1287: F, t8197: F, t1209: F, t8190: F, t1294: F, t7652: F, t1770: F, t2142: F, t1214: F, t7637: F) -> (F, F, F, F, F, F) {
    let t29212 = t8201 * t1248;
    let t29213 = t29212 * t1287;
    let t29216 = t8197 * t1248;
    let t29217 = t29216 * t1287;
    let t29220 = t1209 * t8190;
    let t29224 = t7652 * t8197 * t1294;
    let t29227 = t1770 * t2142;
    let t29233 = t7637 * t8190 * t1214;
    (t29213, t29217, t29220, t29224, t29227, t29233)
}
