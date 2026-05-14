//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1334/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1334<F: Float>(t2258: F, t4313: F, t20998: F, t32260: F, t1486: F, t487: F, t19068: F, t20972: F, t33655: F, t21074: F, t32278: F, t20964: F, t9497: F, t21289: F, t394: F, t9492: F) -> (F, F, F, F, F, F, F) {
    let t113416 = t2258 * t4313;
    let t113419 = t32260 * t20998;
    let t113421 = t1486 * t487;
    let t113422 = t113421 * t19068;
    let t113424 = t33655 * t20972;
    let t113426 = t32278 * t21074;
    let t113428 = t9497 * t20964;
    let t113430 = t21289 * t394;
    let t113431 = t113430 * t9492;
    (t113416, t113419, t113422, t113424, t113426, t113428, t113431)
}
