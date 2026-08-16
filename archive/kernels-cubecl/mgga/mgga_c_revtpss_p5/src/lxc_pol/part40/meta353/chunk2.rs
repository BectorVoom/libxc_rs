//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1214/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1214<F: Float>(t14472: F, t780: F, t2439: F, t212: F, t4469: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F) -> (F, F, F, F) {
    let t14473 = t14472 * t780;
    let t14474 = t2439 * t14473;
    let t14476 = t212 * t4469;
    let t14477 = t14476 * t780;
    let t14479 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t14477;
    let t14480 = t2769 * t1579;
    let t14481 = t14480 * t886;
    let t14482 = t252 * t14481;
    let t14484 = F::cast_from(0.21951497276451705328e-1_f64) * t2782 * t14482;
    let t14485 = t4480 * t2470;
    (t14474, t14479, t14484, t14485)
}
