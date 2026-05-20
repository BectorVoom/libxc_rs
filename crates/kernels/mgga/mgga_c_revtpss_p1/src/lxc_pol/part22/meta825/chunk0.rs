//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2943/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2943<F: Float>(t5552: F, t588: F, t5560: F, t13581: F, t177: F, t762: F, t1317: F, t13632: F, t3857: F, t5569: F, t512: F, t749: F) -> (F, F, F, F, F, F) {
    let t48185 = F::new(32.0) * t5552 * t588;
    let t48212 = F::new(32.0) * t5560 * t588;
    let t48222 = t13581 * t177 * t762;
    let t48225 = t1317 * t13632;
    let t48227 = t3857 * t5569;
    let t48230 = t512 * t13581 * t749;
    (t48185, t48212, t48222, t48225, t48227, t48230)
}
