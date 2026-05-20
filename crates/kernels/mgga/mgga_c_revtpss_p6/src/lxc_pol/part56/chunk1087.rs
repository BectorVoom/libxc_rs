//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1087/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1087<F: Float>(t32655: F, t7741: F, t28042: F, t8692: F, t13426: F, t8460: F, t18227: F, t27123: F, t28219: F, t32392: F, t7742: F, t32394: F) -> (F, F, F, F, F, F, F, F) {
    let t125381 = F::new(4.0) * t32655 * t7741;
    let t125383 = F::new(4.0) * t8692 * t28042;
    let t125384 = t13426 * t8460;
    let t125385 = F::new(2.0) * t125384;
    let t125386 = t18227 * t8460;
    let t125387 = F::new(2.0) * t125386;
    let t125388 = t27123 * t8460;
    let t125389 = F::new(2.0) * t125388;
    let t125390 = t28219 * t8460;
    let t125391 = F::new(2.0) * t125390;
    let t125405 = F::new(4.0) * t32392 * t7742;
    let t125407 = F::new(4.0) * t32394 * t7742;
    (t125381, t125383, t125385, t125387, t125389, t125391, t125405, t125407)
}
