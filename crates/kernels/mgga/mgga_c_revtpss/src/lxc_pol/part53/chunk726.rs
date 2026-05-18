//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 726/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk726<F: Float>(t1294: F, t2142: F, t7652: F, t3140: F, t487: F, t1276: F, t2148: F, t1243: F, t1248: F, t1287: F, t2150: F, t473: F, t7627: F) -> (F, F, F, F, F) {
    let t7653 = t2142 * t1294;
    let t7654 = t7652 * t7653;
    let t7657 = t487 * t3140;
    let t7658 = t7657 * t1276;
    let t7659 = t2148 * t7658;
    let t7660 = t1243 * t2142;
    let t7662 = t7660 * t1248 * t1287;
    let t7666 = t2150 * t473 * t7627;
    (t7654, t7659, t7660, t7662, t7666)
}
