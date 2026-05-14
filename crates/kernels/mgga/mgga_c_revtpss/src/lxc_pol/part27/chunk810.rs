//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 810/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk810<F: Float>(t10626: F, t10627: F, t775: F, t853: F, t2430: F, t10489: F, t832: F, t10618: F, t227: F, t229: F, t2634: F, t2639: F, t2642: F, t4415: F, t830: F, t833: F) -> (F, F) {
    let t10628 = t10626 * t10627;
    let t10631 = t853 * t775;
    let t10632 = t10631 * t2430;
    let t10635 = t832 * t10489;
    let t10638 = -t10618 * t229 + 60.0 * t10628 * t227 - 36.0 * t10632 * t4415 + 3.0 * t10635 * t227 + 9.0 * t2634 * t833 - 36.0 * t2639 * t830 + 9.0 * t2642 * t830;
    (t10631, t10638)
}
