//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2911/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2911<F: Float>(t1386: F, t2237: F, t2482: F, t4021: F, t235: F, t46475: F, t4000: F, t596: F, t10003: F, t4059: F, t9909: F, t72: F, t9940: F) -> (F, F, F, F, F, F, F) {
    let t47198 = t2482 * t1386 * t2237;
    let t47199 = t47198 * t4021;
    let t47201 = t46475 * t235;
    let t47215 = t2482 * t4000 * t596;
    let t47216 = t47215 * t10003;
    let t47229 = t9909 * t4059;
    let t47247 = t9940 * t72;
    (t47198, t47199, t47201, t47215, t47216, t47229, t47247)
}
