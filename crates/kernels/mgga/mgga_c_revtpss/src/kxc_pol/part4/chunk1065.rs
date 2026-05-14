//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1065/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1065<F: Float>(t14476: F, t780: F, t689: F, t1579: F, t2769: F, t886: F, t252: F, t2782: F, t2470: F, t4480: F, t2465: F, t11008: F, t2771: F, t1558: F, t836: F, t231: F) -> (F, F, F, F, F, F) {
    let t14477 = t14476 * t780;
    let t14479 = 0.10975748638225852664e-1 * t689 * t14477;
    let t14480 = t2769 * t1579;
    let t14481 = t14480 * t886;
    let t14482 = t252 * t14481;
    let t14484 = 0.21951497276451705328e-1 * t2782 * t14482;
    let t14485 = t4480 * t2470;
    let t14486 = t2465 * t14485;
    let t14489 = t11008 * t1579 * t2771;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    (t14479, t14484, t14486, t14489, t14494, t14495)
}
