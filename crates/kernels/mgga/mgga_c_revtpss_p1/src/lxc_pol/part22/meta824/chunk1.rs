//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2942/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2942<F: Float>(t13793: F, t13999: F, t1868: F, t3923: F, t13872: F, t221: F, t3978: F, t9921: F, t1320: F, t13632: F, t13672: F, t3860: F, t5567: F) -> (F, F, F, F, F, F) {
    let t48111 = t13999 * t13793;
    let t48113 = t1868 * t3923;
    let t48141 = t221 * t13872;
    let t48143 = t3978 * t9921 * t48141;
    let t48152 = t1320 * t13632;
    let t48154 = t1320 * t13672;
    let t48158 = t3860 * t5567;
    (t48111, t48113, t48143, t48152, t48154, t48158)
}
