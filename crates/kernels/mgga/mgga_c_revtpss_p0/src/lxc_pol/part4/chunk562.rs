//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 562/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk562<F: Float>(t225: F, t2633: F, t73: F, t853: F, t2394: F, t2430: F, t832: F, t227: F, t229: F, t830: F, t833: F) -> (F, F, F, F) {
    let t2634 = t2633 * t225;
    let t2638 = t73 * t853;
    let t2639 = t2638 * t2394;
    let t2642 = t832 * t2430;
    let t2645 = -F::cast_from(12.0_f64) * t227 * t2639 + F::cast_from(3.0_f64) * t227 * t2642 - t229 * t2634 + F::cast_from(6.0_f64) * t830 * t833;
    (t2634, t2639, t2642, t2645)
}
