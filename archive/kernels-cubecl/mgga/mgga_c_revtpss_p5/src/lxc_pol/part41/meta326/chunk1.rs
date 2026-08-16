//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1114/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1114<F: Float>(t14480: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F, t2465: F, t1558: F, t836: F, t231: F, t2797: F, t860: F) -> (F, F, F, F, F) {
    let t14481 = t14480 * t886;
    let t14482 = t252 * t14481;
    let t14484 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t14482;
    let t14485 = t4480 * t2470;
    let t14486 = t2465 * t14485;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    let t14496 = t2797 * t14495;
    let t14498 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14496;
    let t14502 = t860 * t1558;
    (t14484, t14486, t14494, t14498, t14502)
}
