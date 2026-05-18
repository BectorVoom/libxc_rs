//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 459/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk459<F: Float>(t194: F, t618: F, t1412: F, t171: F, t433: F, t1415: F, t385: F, t1413: F, t381: F, t1131: F, t577: F, t155: F) -> (F, F, F, F, F, F) {
    let t5395 = t194 * t618;
    let t5400 = t1412 * t171;
    let t5402 = F::new(0.11696447245269292414e1) * t5400 * t433;
    let t5404 = t385 * t1415;
    let t5407 = F::new(8.0) * t381 * t1413;
    let t5409 = F::new(8.0) * t385 * t1413;
    let t5419 = t577 * t1131;
    let t5420 = t155 * t5419;
    (t5395, t5402, t5404, t5407, t5409, t5420)
}
