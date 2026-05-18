//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 808/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk808<F: Float>(t760: F, t9419: F, t9387: F, t9372: F, t9425: F, t2475: F, t73: F, t2710: F, t2793: F, t9285: F, t874: F, t875: F, t9288: F) -> (F, F, F, F, F, F, F) {
    let t10592 = F::new(0.10389515463408878255e3) * t760 * t9419;
    let t10596 = F::new(0.5848223622634646207e0) * t760 * t9387;
    let t10604 = F::new(0.10254018858216406658e4) * t760 * t9372;
    let t10611 = F::new(0.35089341735807877242e1) * t760 * t9425;
    let t10626 = t73 * t2475;
    let t10645 = F::new(0.46263278077393568556e-2) * t2710 * t2793 * t9285;
    let t10651 = F::new(0.30356481678079769392e-1) * t874 * t875 * t9288;
    (t10592, t10596, t10604, t10611, t10626, t10645, t10651)
}
