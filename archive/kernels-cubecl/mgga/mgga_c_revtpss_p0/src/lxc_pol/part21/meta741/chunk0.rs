//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2607/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2607<F: Float>(t10014: F, t14216: F, t13921: F, t4101: F, t686: F, t72: F, t10139: F, t136: F, t2457: F, t5659: F, t14202: F, t9303: F) -> (F, F, F, F) {
    let t47995 = t10014 * t14216;
    let t47999 = t4101 * t13921 * t72 * t686;
    let t48003 = t10139 * t5659 * t136 * t2457;
    let t48004 = F::cast_from(0.34697458558045176417e-2_f64) * t48003;
    let t48005 = t9303 * t14202;
    (t47995, t47999, t48004, t48005)
}
