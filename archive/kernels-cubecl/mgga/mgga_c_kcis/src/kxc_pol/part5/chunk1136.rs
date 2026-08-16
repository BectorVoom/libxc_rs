//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1136/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1136<F: Float>(t1020: F, t19123: F, t3182: F, t6491: F, t1096: F, t1092: F, t6621: F, t9429: F, t9425: F, t1646: F, t4772: F, t3203: F) -> (F, F, F, F, F) {
    let t19124 = t1020 * t19123;
    let t19126 = t3182 * t6491;
    let t19127 = t1096 * t19126;
    let t19128 = t1092 * t19127;
    let t19130 = t9429 * t6621;
    let t19132 = t9425 * t6621;
    let t19134 = t1646 * t4772;
    let t19135 = t3203 * t19134;
    (t19124, t19128, t19130, t19132, t19135)
}
