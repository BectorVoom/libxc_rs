//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2673/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2673<F: Float>(t1065: F, t372: F, t6305: F, t19912: F, t3241: F, t1011: F, t6292: F, t697: F, t11922: F, t19717: F, t4899: F, t11675: F, t19785: F) -> (F, F, F, F, F) {
    let t66187 = t372 * t1065 * t6305;
    let t66215 = t3241 * t19912;
    let t66218 = t1011 * t697 * t6292;
    let t66221 = t4899 * t11922 * t19717;
    let t66261 = t11675 * t19785;
    (t66187, t66215, t66218, t66221, t66261)
}
