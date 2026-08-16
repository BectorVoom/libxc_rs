//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1249/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1249<F: Float>(t587: F, t65: F, t197: F, t532: F, t1450: F, t2033: F, t143: F, t2580: F, t130: F, t2566: F, t700: F, t2584: F) -> (F, F, F, F, F, F) {
    let t8779 = F::cast_from(1.0_f64) / t65 / t587;
    let t8995 = t197 * t532;
    let t8996 = t2033 * t1450;
    let t9273 = F::cast_from(1.0_f64) / t2580 / t143;
    let t9274 = t130 * t9273;
    let t9275 = t2566 * t700;
    let t9276 = t9275 * t2584;
    (t8779, t8995, t8996, t9274, t9275, t9276)
}
