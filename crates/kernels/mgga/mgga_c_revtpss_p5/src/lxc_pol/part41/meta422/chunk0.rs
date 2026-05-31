//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1479/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1479<F: Float>(t1518: F, t2178: F, t2681: F, t64: F, t10207: F, t111: F, t116: F, t21813: F, t5876: F, t670: F, t5891: F, t665: F) -> (F, F, F, F, F, F) {
    let t35739 = t1518 * t2178;
    let t46089 = t64 * t2681;
    let t46157 = F::cast_from(1.0_f64) / t10207 / t111;
    let t75439 = t21813 * t116;
    let t85360 = t5876 * t670;
    let t105872 = t5891 * t665;
    (t35739, t46089, t46157, t75439, t85360, t105872)
}
