//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1212/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1212<F: Float>(t11510: F, t40549: F, t23754: F, t3263: F, t3275: F, t10810: F, t3429: F, t3692: F, t10935: F, t2816: F, t3446: F, t10928: F, t122: F, t3434: F, t874: F, t955: F) -> (F, F, F, F, F) {
    let t40551 = F::cast_from(3.0_f64) * t40549 * t11510;
    let t40554 = t3275 * t3263 * t23754 / F::cast_from(4.0_f64);
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    let t40560 = F::cast_from(0.19211284388664477842e-2_f64) * t40559;
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    (t40551, t40554, t40556, t40560, t40564)
}
