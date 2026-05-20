//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2567/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2567<F: Float>(t3853: F, t3857: F, t820: F, t843: F, t9991: F, t9997: F, t1386: F, t2237: F, t2482: F, t4021: F, t235: F, t46475: F) -> (F, F, F, F, F) {
    let t47152 = F::new(120.0) * t3857 * t3853;
    let t47194 = t820 * t9991 * t843;
    let t47195 = t47194 * t9997;
    let t47198 = t2482 * t1386 * t2237;
    let t47199 = t47198 * t4021;
    let t47201 = t46475 * t235;
    (t47152, t47195, t47198, t47199, t47201)
}
