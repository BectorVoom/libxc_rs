//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1091/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1091<F: Float>(t5916: F, t625: F, t10227: F, t5895: F, t10241: F, t5907: F, t6785: F, t9335: F, t6792: F, t9350: F, t1450: F, t6922: F) -> (F, F, F, F, F, F) {
    let t21827 = t625 * t5916;
    let t21835 = t10227 * t5895;
    let t21860 = t10241 * t5907;
    let t21906 = t9335 * t6785;
    let t21918 = t9350 * t6792;
    let t21937 = t6922 * t1450;
    (t21827, t21835, t21860, t21906, t21918, t21937)
}
