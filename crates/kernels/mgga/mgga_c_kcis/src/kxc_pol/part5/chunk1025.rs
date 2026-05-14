//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1025/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1025<F: Float>(t18594: F, t18690: F, t18772: F, t19110: F, t1022: F, t1096: F, t1092: F, t4985: F, t5026: F, t4814: F, t4999: F, t5005: F, t1020: F, t3182: F, t6491: F, t6621: F, t9429: F) -> (F, F, F, F, F, F, F) {
    let t19112 = t18594 + t18690 + t18772 + t19110;
    let t19113 = t1022 * t19112;
    let t19114 = t1096 * t19113;
    let t19115 = t1092 * t19114;
    let t19117 = t5026 * t4985;
    let t19118 = t1092 * t19117;
    let t19120 = t4999 * t4814;
    let t19121 = t1092 * t19120;
    let t19123 = t4999 * t5005;
    let t19124 = t1020 * t19123;
    let t19126 = t3182 * t6491;
    let t19127 = t1096 * t19126;
    let t19128 = t1092 * t19127;
    let t19130 = t9429 * t6621;
    (t19112, t19115, t19118, t19121, t19124, t19128, t19130)
}
