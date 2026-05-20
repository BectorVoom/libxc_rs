//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1989/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1989<F: Float>(t13944: F, t5673: F, t5675: F, t5674: F, t9955: F, t9956: F, t4000: F, t820: F, t844: F) -> (F, F, F) {
    let t13991 = t5673 * t13944 * t5675;
    let t13995 = t9955 * t5674 * t9956;
    let t13999 = t820 * t4000 * t844;
    (t13991, t13995, t13999)
}
