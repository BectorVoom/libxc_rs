//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1492/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1492<F: Float>(t2470: F, t4480: F, t2465: F, t11008: F, t1579: F, t2771: F, t1558: F, t836: F, t231: F, t2797: F, t2782: F, t860: F) -> (F, F, F, F, F, F, F) {
    let t14485 = t4480 * t2470;
    let t14486 = t2465 * t14485;
    let t14489 = t11008 * t1579 * t2771;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    let t14496 = t2797 * t14495;
    let t14498 = F::cast_from(0.10975748638225852664e-1_f64) * t2782 * t14496;
    let t14502 = t860 * t1558;
    (t14485, t14486, t14489, t14494, t14495, t14498, t14502)
}
